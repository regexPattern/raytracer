use std::num::NonZeroUsize;

use indicatif::{ProgressBar, ProgressStyle};
use thiserror::Error;

use crate::{
    scene::SceneProgress,
    shape::{Group, Shape, SmoothTriangle, Triangle},
    tuple::{Point, Vector}, transform::Transform,
};

const MIN_POLYGON_VERTICES: usize = 3;

#[derive(Debug, PartialEq)]
pub struct OBJModel {
    groups: Vec<PolygonGroup>,
    normals: Vec<Vector>,
    vertices: Vec<Point>,
    parsing_progress: SceneProgress,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct FaceVertex {
    vertex: Point,
    normal: Option<Vector>,
}

#[derive(Debug, PartialEq)]
struct PolygonGroup {
    group: Group,
    name: String,
}

#[derive(Debug, Error, PartialEq)]
#[error("parsing error at line {}: '{kind}'", line_nr + 1)]
pub struct ParsingError {
    kind: ParsingErrorKind,
    line_nr: usize,
}

#[derive(Debug, Error, PartialEq)]
pub enum ParsingErrorKind {
    #[error(transparent)]
    InvalidCoordinate(#[from] std::num::ParseFloatError),

    #[error(transparent)]
    InvalidVertexIndex(#[from] std::num::ParseIntError),

    #[error("insufficient vertices for a polygon")]
    InsufficientVertices,

    #[error("no element at index: `{accessed}` out of `{available}` available (1-indexed)")]
    FaceElementOutOfBounds {
        accessed: NonZeroUsize,
        available: usize,
    },

    #[error("missing field: `{name}`")]
    MissingField { name: &'static str },
}

impl OBJModel {
    pub fn new(string: &str, parse_progress: SceneProgress) -> Result<Self, ParsingError> {
        let mut groups = vec![PolygonGroup {
            group: Group::default(),
            name: "__default".to_string(),
        }];

        let mut normals = vec![];
        let mut vertices = vec![];

        let progress_bar = if let SceneProgress::Enable = parse_progress {
            let bar = ProgressBar::new_spinner();

            // This style template string is ensured to be valid.
            #[allow(clippy::unwrap_used)]
            bar.set_style(
                ProgressStyle::with_template("{spinner} loading OBJ model ({pos} lines read)")
                    .unwrap(),
            );
            bar
        } else {
            ProgressBar::hidden()
        };

        for (line_nr, line) in string.lines().enumerate() {
            let propagate_line_err = |kind| ParsingError { kind, line_nr };

            if line.starts_with("v ") {
                let (x, y, z) = parse_coordinate(line).map_err(propagate_line_err)?;
                vertices.push(Point::new(x, y, z));
            } else if line.starts_with("f ") {
                let face = parse_face(line, &normals, &vertices).map_err(propagate_line_err)?;

                // There's always going to be a valid group in the group's queue, as it always
                // contains at least the "__default" group.
                #[allow(clippy::unwrap_used)]
                groups.last_mut().unwrap().group.extend(face);
            } else if line.starts_with("g ") {
                groups.push(parse_group(line).map_err(propagate_line_err)?);
            } else if line.starts_with("vn") {
                let (x, y, z) = parse_coordinate(line).map_err(propagate_line_err)?;
                normals.push(Vector::new(x, y, z));
            }

            progress_bar.inc(1);
        }

        Ok(OBJModel {
            groups,
            normals,
            vertices,
            parsing_progress: parse_progress,
        })
    }

    pub fn build(self, transform: Transform) -> Group {
        let groups: Vec<_> = self
            .groups
            .into_iter()
            .map(|group| Shape::Group(group.group))
            .collect();

        Group::new(groups, transform)
    }
}

fn parse_coordinate(line: &str) -> Result<(f64, f64, f64), ParsingErrorKind> {
    let mut fields = line.split_whitespace().skip(1);

    let x = fields
        .next()
        .ok_or(ParsingErrorKind::MissingField { name: "x" })?
        .parse::<f64>()?;

    let y = fields
        .next()
        .ok_or(ParsingErrorKind::MissingField { name: "y" })?
        .parse::<f64>()?;

    let z = fields
        .next()
        .ok_or(ParsingErrorKind::MissingField { name: "z" })?
        .parse::<f64>()?;

    Ok((x, y, z))
}

fn parse_face(
    line: &str,
    saved_normals: &[Vector],
    saved_vertices: &[Point],
) -> Result<Vec<Shape>, ParsingErrorKind> {
    let elements: Vec<_> = line.split_whitespace().skip(1).collect();

    if elements.len() < MIN_POLYGON_VERTICES {
        return Err(ParsingErrorKind::InsufficientVertices);
    }

    let mut vertices = vec![];

    for elem in elements {
        let mut fields = elem.split('/');

        // There's always going to be an element in the split's first position. This element might
        // be empty, but it is there. Empty elements are going to be handled when parsing them into
        // numbers from `get_face_element()`.
        #[allow(clippy::unwrap_used)]
        let vertex = get_face_element(fields.next().unwrap(), saved_vertices)?;

        fields.next();

        let normal = match fields.next() {
            Some(normal_index) => Some(get_face_element(normal_index, saved_normals)?),
            None => None,
        };

        vertices.push(FaceVertex { vertex, normal });
    }

    fan_triangulation(vertices)
}

fn get_face_element<T>(raw: &str, saved_elements: &[T]) -> Result<T, ParsingErrorKind>
where
    T: Copy,
{
    let index = raw.parse::<NonZeroUsize>()?;
    saved_elements
        .get(index.get() - 1)
        .ok_or(ParsingErrorKind::FaceElementOutOfBounds {
            accessed: index,
            available: saved_elements.len(),
        })
        .copied()
}

fn fan_triangulation(vertices: Vec<FaceVertex>) -> Result<Vec<Shape>, ParsingErrorKind> {
    let mut triangles = vec![];

    for i in 2..vertices.len() {
        let v0 = vertices[0];
        let v1 = vertices[i - 1];
        let v2 = vertices[i];

        if let Ok(triangle) = Triangle::try_default([v0.vertex, v1.vertex, v2.vertex]) {
            let triangle = if let (Some(n0), Some(n1), Some(n2)) = (v0.normal, v1.normal, v2.normal)
            {
                Shape::SmoothTriangle(SmoothTriangle {
                    triangle,
                    n0,
                    n1,
                    n2,
                })
            } else {
                Shape::Triangle(triangle)
            };

            triangles.push(triangle);
        }
    }

    Ok(triangles)
}

fn parse_group(line: &str) -> Result<PolygonGroup, ParsingErrorKind> {
    let group_name = line
        .split_whitespace()
        .nth(1)
        .ok_or(ParsingErrorKind::MissingField { name: "group_name" })?;

    Ok(PolygonGroup {
        group: Group::default(),
        name: group_name.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_vertex_records() {
        let input = "\
v -1 1 0
v -1.0000 0.50000 0.0000
v 1 0 0
v 1 1 0";

        let model = OBJModel::new(input, SceneProgress::Disable).unwrap();

        assert_eq!(model.vertices[0], Point::new(-1.0, 1.0, 0.0));
        assert_eq!(model.vertices[1], Point::new(-1.0, 0.5, 0.0));
        assert_eq!(model.vertices[2], Point::new(1.0, 0.0, 0.0));
        assert_eq!(model.vertices[3], Point::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn parsing_a_vertex() {
        let input = "v 1 2.5000 -3.0";

        let vertex = parse_coordinate(input).unwrap();

        assert_eq!(vertex, (1.0, 2.5, -3.0));
    }

    #[test]
    fn trying_to_parse_a_vertex_with_a_missing_field() {
        assert_eq!(
            parse_coordinate("v "),
            Err(ParsingErrorKind::MissingField { name: "x" })
        );

        assert_eq!(
            parse_coordinate("v 1"),
            Err(ParsingErrorKind::MissingField { name: "y" })
        );

        assert_eq!(
            parse_coordinate("v 1 2.5"),
            Err(ParsingErrorKind::MissingField { name: "z" })
        );
    }

    #[test]
    fn trying_to_parse_a_vertex_with_an_invalid_coordinate() {
        assert!(matches!(
            parse_coordinate("v 1 @ 2.0"),
            Err(ParsingErrorKind::InvalidCoordinate(_))
        ));
    }

    #[test]
    fn getting_error_with_line_information_when_parsing_fails() {
        let input = "v 1";

        assert_eq!(
            OBJModel::new(input, SceneProgress::Disable),
            Err(ParsingError {
                kind: ParsingErrorKind::MissingField { name: "y" },
                line_nr: 0,
            })
        );
    }

    #[test]
    fn an_error_should_display_with_correct_message() {
        let input = "v 1";

        let err = OBJModel::new(input, SceneProgress::Disable).unwrap_err();

        assert_eq!(
            err.to_string(),
            "parsing error at line 1: 'missing field: `y`'"
        );
    }

    #[test]
    fn parsing_triangle_faces() {
        let input = "\
v -1 1 0
v -2 0 0
v 1 0 0
v 1 1 0

f 1 2 3
f 1 3 4";

        let model = OBJModel::new(input, SceneProgress::Disable).unwrap();

        let g = &model.groups[0].group;
        let t0 = &g.children[0];
        let t1 = &g.children[1];

        assert_eq!(
            t0,
            &Shape::Triangle(
                Triangle::try_default([model.vertices[0], model.vertices[1], model.vertices[2]])
                    .unwrap()
            )
        );

        assert_eq!(
            t1,
            &Shape::Triangle(
                Triangle::try_default([model.vertices[0], model.vertices[2], model.vertices[3]])
                    .unwrap()
            )
        );
    }

    #[test]
    fn trying_to_parse_a_face_with_insufficient_vertices() {
        let input = "f ";

        let err = parse_face(input, &[], &[]).unwrap_err();

        assert_eq!(err, ParsingErrorKind::InsufficientVertices);
    }

    #[test]
    fn trying_to_parse_a_face_element_with_an_invalid_vertex_index() {
        assert!(matches!(
            get_face_element::<Point>("@", &[]),
            Err(ParsingErrorKind::InvalidVertexIndex(_))
        ));

        let err = get_face_element("2", &[Point::new(1.0, 2.0, 3.0)]).unwrap_err();

        assert_eq!(
            err,
            ParsingErrorKind::FaceElementOutOfBounds {
                accessed: NonZeroUsize::new(2).unwrap(),
                available: 1,
            }
        );

        assert_eq!(
            err.to_string(),
            "no element at index: `2` out of `1` available (1-indexed)"
        );
    }

    #[test]
    fn parsing_a_face_element() {
        let vertices = [
            Point::new(1.0, 2.0, 3.0),
            Point::new(2.0, 3.0, 4.0),
            Point::new(3.0, 4.0, 5.0),
            Point::new(4.0, 5.0, 6.0),
        ];

        let vertex = get_face_element("3", &vertices).unwrap();

        assert_eq!(vertex, vertices[2]);
    }

    #[test]
    fn parsing_a_single_triangle_face() {
        let vertices = [
            Point::new(2.0, 5.0, 1.0),
            Point::new(7.0, -2.0, 3.0),
            Point::new(4.0, 1.5, 4.25),
        ];

        let input = "f 1 2 3";

        let tri = parse_face(input, &[], &vertices).unwrap();

        assert_eq!(
            tri[0],
            Shape::Triangle(Triangle::try_default(vertices).unwrap())
        );
    }

    #[test]
    fn triangulating_polygons() {
        let input = "\
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
v 0 2 0

f 1 2 3 4 5";

        let model = OBJModel::new(input, SceneProgress::Disable).unwrap();

        let g = &model.groups[0].group;
        let t0 = &g.children[0];
        let t1 = &g.children[1];
        let t2 = &g.children[2];

        assert_eq!(
            t0,
            &Shape::Triangle(
                Triangle::try_default([model.vertices[0], model.vertices[1], model.vertices[2]])
                    .unwrap()
            )
        );

        assert_eq!(
            t1,
            &Shape::Triangle(
                Triangle::try_default([model.vertices[0], model.vertices[2], model.vertices[3]])
                    .unwrap()
            )
        );

        assert_eq!(
            t2,
            &Shape::Triangle(
                Triangle::try_default([model.vertices[0], model.vertices[3], model.vertices[4]])
                    .unwrap()
            )
        );
    }

    #[test]
    fn triangles_in_groups() {
        let input = "\
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
g FirstGroup
f 1 2 3
g SecondGroup
f 1 3 4";

        let model = OBJModel::new(input, SceneProgress::Disable).unwrap();

        let g1 = &model
            .groups
            .iter()
            .find(|polygon_group| polygon_group.name == "FirstGroup")
            .unwrap()
            .group;

        let g2 = &model
            .groups
            .iter()
            .find(|polygon_group| polygon_group.name == "SecondGroup")
            .unwrap()
            .group;

        let t0 = &g1.children[0];
        let t1 = &g2.children[0];

        assert_eq!(
            t0,
            &Shape::Triangle(
                Triangle::try_default([model.vertices[0], model.vertices[1], model.vertices[2]])
                    .unwrap()
            )
        );

        assert_eq!(
            t1,
            &Shape::Triangle(
                Triangle::try_default([model.vertices[0], model.vertices[2], model.vertices[3]])
                    .unwrap()
            )
        );
    }

    #[test]
    fn trying_to_parse_a_group_without_a_name() {
        assert_eq!(
            parse_group("g"),
            Err(ParsingErrorKind::MissingField { name: "group_name" })
        );
    }

    #[test]
    fn converting_an_obj_model_to_a_group() {
        let input = "\
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
g FirstGroup
f 1 2 3
g SecondGroup
f 1 3 4";

        let model = OBJModel::new(input, SceneProgress::Disable).unwrap();
        let first_group = Shape::Group(model.groups[1].group.clone());
        let second_group = Shape::Group(model.groups[2].group.clone());

        let g = model.build(Default::default());

        assert!(g.children.contains(&first_group));
        assert!(g.children.contains(&second_group));
    }

    #[test]
    fn parsing_vertex_normal_records() {
        let input = r"\
vn 0 0 1
vn 0.707 0 -0.707
vn 1 2 3";

        let model = OBJModel::new(input, SceneProgress::Disable).unwrap();

        assert_eq!(model.normals[0], Vector::new(0.0, 0.0, 1.0));
        assert_eq!(model.normals[1], Vector::new(0.707, 0.0, -0.707));
        assert_eq!(model.normals[2], Vector::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn parsing_faces_with_normals() {
        let input = r"\
v 0 1 0
v -1 0 0
v 1 0 0

vn -1 0 0
vn 1 0 0
vn 0 1 0

f 1//3 2//1 3//2
f 1/0/3 2/102/1 3/14/2";

        let model = OBJModel::new(input, SceneProgress::Disable).unwrap();

        let g = &model.groups[0].group;
        let t0 = &g.children[0];
        let t1 = &g.children[1];

        assert_eq!(
            t0,
            &Shape::SmoothTriangle(SmoothTriangle {
                triangle: Triangle::try_default([
                    model.vertices[0],
                    model.vertices[1],
                    model.vertices[2]
                ])
                .unwrap(),
                n0: model.normals[2],
                n1: model.normals[0],
                n2: model.normals[1],
            })
        );

        assert_eq!(t1, t0);
    }

    #[test]
    fn parsing_a_single_smooth_triangle_face() {
        let normals = [
            Vector::new(2.0, 5.0, 1.0),
            Vector::new(7.0, -2.0, 3.0),
            Vector::new(4.0, 1.5, 4.25),
        ];

        let vertices = [
            Point::new(2.0, 5.0, 1.0),
            Point::new(7.0, -2.0, 3.0),
            Point::new(4.0, 1.5, 4.25),
        ];

        let input = "f 1//3 2//2 3//1";

        let tri = parse_face(input, &normals, &vertices).unwrap();

        assert_eq!(
            tri[0],
            Shape::SmoothTriangle(SmoothTriangle {
                triangle: Triangle::try_default(vertices).unwrap(),
                n0: normals[2],
                n1: normals[1],
                n2: normals[0]
            })
        );
    }
}
