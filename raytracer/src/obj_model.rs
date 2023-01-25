use std::{cell::RefCell, num::NonZeroUsize};

use crate::{
    shape::{CollinearTriangleSidesError, Group, Shape, SmoothTriangle, Triangle},
    tuple::{Point, Vector},
};

#[derive(Debug, PartialEq)]
pub struct ParsingError<'a> {
    kind: ParsingErrorKind,
    line: LineInfo<'a>,
}

#[derive(Debug, PartialEq)]
pub struct OBJModel {
    groups: Vec<PolygonsGroup>,
    vertices: Vec<Point>,
    normals: Vec<Vector>,
}

#[derive(Debug, PartialEq)]
struct PolygonsGroup {
    name: String,
    group: Group,
}

#[derive(Copy, Clone, Debug)]
struct FaceVertex {
    vertex: Point,
    normal: Option<Vector>,
}

#[derive(Debug, PartialEq)]
pub enum ParsingErrorKind {
    InsufficientVertices,
    InvalidPolygon,
    InvalidValue,
    MissingValue,
}

#[derive(Debug, PartialEq)]
pub struct LineInfo<'a> {
    number: usize,
    raw_data: &'a str,
}

impl From<CollinearTriangleSidesError> for ParsingErrorKind {
    fn from(_: CollinearTriangleSidesError) -> Self {
        Self::InvalidPolygon
    }
}

impl From<OBJModel> for Group {
    fn from(value: OBJModel) -> Self {
        let children: Vec<_> = value
            .groups
            .into_iter()
            .map(|group| Shape::Group(group.group))
            .collect();

        Self::new(children, Default::default())
    }
}

impl OBJModel {
    pub fn import(text: &str) -> Result<Self, ParsingError> {
        let mut groups = vec![RefCell::new(PolygonsGroup {
            name: "__default".to_string(),
            group: Default::default(),
        })];

        let mut declared_vertices = vec![];
        let mut declared_normals = vec![];

        for (i, line) in text.lines().enumerate() {
            // Some OBJ files use two spaces for indentation between fields.
            let sanitized = line.replace("  ", " ");

            let wrap_parsing_error = |kind| ParsingError {
                kind,
                line: LineInfo {
                    number: i + 1,
                    raw_data: line,
                },
            };

            if sanitized.starts_with("v ") {
                let (x, y, z) = parse_coordinate(&sanitized).map_err(wrap_parsing_error)?;
                declared_vertices.push(Point::new(x, y, z));
            } else if sanitized.starts_with("f ") {
                // Skip triangles with collinear sides. This issue is usually encountered in file
                // with polygons with four or more sides.
                if let Ok(triangles) = parse_face(&sanitized, &declared_vertices, &declared_normals)
                {
                    // There's always an available group, starting with the __default one.
                    #[allow(clippy::unwrap_used)]
                    groups
                        .last()
                        .unwrap()
                        .borrow_mut()
                        .group
                        .add_children(triangles);
                }
            } else if sanitized.starts_with("g ") {
                let name = parse_group_name(&sanitized).map_err(wrap_parsing_error)?;
                groups.push(RefCell::new(PolygonsGroup {
                    name,
                    group: Default::default(),
                }));
            } else if sanitized.starts_with("vn") {
                let (x, y, z) = parse_coordinate(&sanitized).map_err(wrap_parsing_error)?;
                declared_normals.push(Vector::new(x, y, z));
            }
        }

        let groups = groups.into_iter().map(|group| group.into_inner()).collect();

        Ok(Self {
            groups,
            vertices: declared_vertices,
            normals: declared_normals,
        })
    }

    pub fn load(self) -> Shape {
        Shape::Group(Group::from(self))
    }
}

fn parse_value<T>(sanitized: Option<&str>) -> Result<T, ParsingErrorKind>
where
    T: std::str::FromStr,
{
    sanitized
        .ok_or(ParsingErrorKind::MissingValue)?
        .parse::<T>()
        .map_err(|_| ParsingErrorKind::InvalidValue)
}

fn parse_coordinate(sanitized: &str) -> Result<(f64, f64, f64), ParsingErrorKind> {
    let mut split = sanitized.split(' ').skip(1);

    let x = parse_value::<f64>(split.next())?;
    let y = parse_value::<f64>(split.next())?;
    let z = parse_value::<f64>(split.next())?;

    Ok((x, y, z))
}

fn parse_face(
    sanitized: &str,
    declared_vertices: &[Point],
    declared_normals: &[Vector],
) -> Result<Vec<Shape>, ParsingErrorKind> {
    if declared_vertices.len() < 3 {
        return Err(ParsingErrorKind::InsufficientVertices);
    }

    let mut face_vertices = vec![];
    let fields = sanitized.split(' ').skip(1);

    for field in fields {
        let mut vertex_data = field.split('/');

        let vertex_index = parse_value::<NonZeroUsize>(vertex_data.nth(0))?.get() - 1;
        let normal_index = parse_value::<NonZeroUsize>(vertex_data.nth(1))
            .map(|index| index.get() - 1)
            .ok();

        if let Some(vertex) = declared_vertices.get(vertex_index).copied() {
            let normal = match normal_index {
                Some(index) => declared_normals.get(index).copied(),
                None => None,
            };

            face_vertices.push(FaceVertex { vertex, normal });
        }
    }

    fan_triangulation(&face_vertices)
}

fn fan_triangulation(face_vertices: &[FaceVertex]) -> Result<Vec<Shape>, ParsingErrorKind> {
    let mut triangles = vec![];

    for i in 2..face_vertices.len() {
        let face_vertices = [face_vertices[0], face_vertices[i - 1], face_vertices[i]];
        let vertices = (
            face_vertices[0].vertex,
            face_vertices[1].vertex,
            face_vertices[2].vertex,
        );
        let normals = [
            face_vertices[0].normal,
            face_vertices[1].normal,
            face_vertices[2].normal,
        ];

        let triangle = Triangle::try_new(
            Default::default(),
            Default::default(),
            [vertices.0, vertices.1, vertices.2],
        )
        .map_err(|_| ParsingErrorKind::InvalidPolygon)?;

        if normals.iter().all(|vertex| vertex.is_some()) {
            triangles.push(Shape::SmoothTriangle(SmoothTriangle {
                triangle,
                n0: normals[0].unwrap(),
                n1: normals[1].unwrap(),
                n2: normals[2].unwrap(),
            }));
        } else {
            triangles.push(Shape::Triangle(triangle));
        }
    }

    Ok(triangles)
}

fn parse_group_name(sanitized: &str) -> Result<String, ParsingErrorKind> {
    sanitized
        .split(' ')
        .nth(1)
        .map(|name| name.to_string())
        .ok_or(ParsingErrorKind::MissingValue)
}

#[cfg(test)]
mod tests {
    use crate::{assert_approx, shape::SmoothTriangle};

    use super::*;

    #[test]
    fn parsing_a_missing_value() {
        assert_eq!(
            super::parse_value::<f64>(None),
            Err(ParsingErrorKind::MissingValue)
        );
    }

    #[test]
    fn parsing_an_invalid_value() {
        assert_eq!(
            super::parse_value::<f64>(Some("@")),
            Err(ParsingErrorKind::InvalidValue)
        );
    }

    #[test]
    fn parsing_a_value() {
        let value = super::parse_value::<f64>(Some("69.420")).unwrap();

        assert_approx!(value, 69.420);
    }

    #[test]
    fn parsing_a_coordinate() {
        let point = super::parse_coordinate("v 10.5 -1 0").unwrap();
        let invalid_value = super::parse_coordinate("v 1 @ 3");
        let missing_value = super::parse_coordinate("v 1 2");

        let point = Point::new(point.0, point.1, point.2);
        assert_eq!(point, Point::new(10.5, -1.0, 0.0));

        assert_eq!(invalid_value, Err(ParsingErrorKind::InvalidValue));
        assert_eq!(missing_value, Err(ParsingErrorKind::MissingValue));
    }

    #[test]
    fn parsing_vertex_records() {
        let input = "\
v -1 1 0
v -1.0000 0.5000 0.0000
v 1 0 0
v 1 1 0";

        let model = OBJModel::import(&input).unwrap();

        assert_eq!(model.vertices[0], Point::new(-1.0, 1.0, 0.0));
        assert_eq!(model.vertices[1], Point::new(-1.0, 0.5, 0.0));
        assert_eq!(model.vertices[2], Point::new(1.0, 0.0, 0.0));
        assert_eq!(model.vertices[3], Point::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn parsing_a_zero_usize() {
        assert_eq!(
            super::parse_value::<NonZeroUsize>(Some("0")),
            Err(ParsingErrorKind::InvalidValue)
        );
    }

    #[test]
    fn parsing_a_polygon() {
        let defined_vertices = [
            Point::new(0.0, 1.0, 2.0),
            Point::new(3.0, 4.0, 5.0),
            Point::new(-1.0, -2.0, 3.0),
        ];

        let polygon = super::parse_face("f 1 2 3", &defined_vertices, &[]).unwrap();

        assert_eq!(
            polygon[0],
            Shape::Triangle(
                Triangle::try_new(
                    Default::default(),
                    Default::default(),
                    [
                        defined_vertices[0],
                        defined_vertices[1],
                        defined_vertices[2]
                    ]
                )
                .unwrap()
            )
        );
    }

    #[test]
    fn parsing_triangle_faces() {
        let input = "\
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

f 1 2 3
f 1 3 4";

        let model = OBJModel::import(&input).unwrap();

        let g = &model.groups[0];

        let t0 = &g.group.children[0];
        let t1 = &g.group.children[1];

        assert_eq!(
            t0,
            &Shape::Triangle(
                Triangle::try_new(
                    Default::default(),
                    Default::default(),
                    [model.vertices[0], model.vertices[1], model.vertices[2]],
                )
                .unwrap(),
            )
        );

        assert_eq!(
            t1,
            &Shape::Triangle(
                Triangle::try_new(
                    Default::default(),
                    Default::default(),
                    [model.vertices[0], model.vertices[2], model.vertices[3]],
                )
                .unwrap(),
            )
        );
    }

    #[test]
    fn trying_to_parse_an_invalid_polygon() {
        let defined_vertices = [
            Point::new(1.0, 1.0, 1.0),
            Point::new(2.0, 2.0, 2.0),
            Point::new(3.0, 3.0, 3.0),
        ];

        assert_eq!(
            super::parse_face("f 1 2 3", &defined_vertices, &[]),
            Err(ParsingErrorKind::InvalidPolygon)
        );
    }

    #[test]
    fn trying_to_parse_a_polygon_with_less_than_3_vertices_defined() {
        let defined_vertices = [Point::new(1.0, 1.0, 1.0), Point::new(2.0, 2.0, 2.0)];

        assert_eq!(
            super::parse_face("f 1 2 3", &defined_vertices, &[]),
            Err(ParsingErrorKind::InsufficientVertices)
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

        let model = OBJModel::import(&input).unwrap();

        let g = &model.groups[0];

        let t0 = &g.group.children[0];
        let t1 = &g.group.children[1];
        let t2 = &g.group.children[2];

        let expected_t0 = Shape::Triangle(
            Triangle::try_new(
                Default::default(),
                Default::default(),
                [model.vertices[0], model.vertices[1], model.vertices[2]],
            )
            .unwrap(),
        );

        let expected_t1 = Shape::Triangle(
            Triangle::try_new(
                Default::default(),
                Default::default(),
                [model.vertices[0], model.vertices[2], model.vertices[3]],
            )
            .unwrap(),
        );

        let expected_t2 = Shape::Triangle(
            Triangle::try_new(
                Default::default(),
                Default::default(),
                [model.vertices[0], model.vertices[3], model.vertices[4]],
            )
            .unwrap(),
        );

        assert_eq!(t0, &expected_t0);
        assert_eq!(t1, &expected_t1);
        assert_eq!(t2, &expected_t2);
    }

    #[test]
    fn triangles_in_groups() {
        let input = r"\
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

f 4 1 2

g FirstGroup
f 3 2 1

g SecondGroup
f 1 2 4";

        let model = OBJModel::import(&input).unwrap();

        let default_group = &model.groups[0];
        let first_group = &model.groups[1];
        let second_group = &model.groups[2];

        let v0 = model.vertices[0];
        let v1 = model.vertices[1];
        let v2 = model.vertices[2];
        let v3 = model.vertices[3];

        assert_eq!(
            default_group,
            &PolygonsGroup {
                name: "__default".to_string(),
                group: Group::new(
                    [Shape::Triangle(
                        Triangle::try_new(Default::default(), Default::default(), [v3, v0, v1])
                            .unwrap()
                    )],
                    Default::default()
                )
            }
        );

        assert_eq!(
            first_group,
            &PolygonsGroup {
                name: "FirstGroup".to_string(),
                group: Group::new(
                    [Shape::Triangle(
                        Triangle::try_new(Default::default(), Default::default(), [v2, v1, v0])
                            .unwrap()
                    ),],
                    Default::default()
                )
            }
        );

        assert_eq!(
            second_group,
            &PolygonsGroup {
                name: "SecondGroup".to_string(),
                group: Group::new(
                    [Shape::Triangle(
                        Triangle::try_new(Default::default(), Default::default(), [v0, v1, v3])
                            .unwrap()
                    ),],
                    Default::default()
                )
            }
        );
    }

    #[test]
    fn converting_an_obj_file_to_a_group() {
        let input = r"\
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

g FirstGroup
f 3 2 1

g SecondGroup
f 1 2 4";

        let model = OBJModel::import(&input).unwrap();

        let first_group = Shape::Group(model.groups[1].group.clone());
        let second_group = Shape::Group(model.groups[2].group.clone());

        let g = Group::from(model);

        assert!(g.children.contains(&first_group));
        assert!(g.children.contains(&second_group));
    }

    #[test]
    fn vertex_normal_records() {
        let input = r"\
vn 0 0 1
vn 0.707 0 -0.707
vn 1 2 3";

        let model = OBJModel::import(&input).unwrap();

        assert_eq!(model.normals[0], Vector::new(0.0, 0.0, 1.0));
        assert_eq!(model.normals[1], Vector::new(0.707, 0.0, -0.707));
        assert_eq!(model.normals[2], Vector::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn faces_with_normals() {
        let input = r"\
v 0 1 0
v -1 0 0
v 1 0 0

vn -1 0 0
vn 1 0 0
vn 0 1 0

f 1//3 2//1 3//2
f 1/0/3 2/102/1 3/14/2";

        let model = OBJModel::import(&input).unwrap();

        let default_group = &model.groups[0];
        let t0 = &default_group.group.children[0];
        let t1 = &default_group.group.children[1];

        assert_eq!(
            t0,
            &Shape::SmoothTriangle(SmoothTriangle {
                triangle: Triangle::try_new(
                    Default::default(),
                    Default::default(),
                    [model.vertices[0], model.vertices[1], model.vertices[2],]
                )
                .unwrap(),
                n0: model.normals[2],
                n1: model.normals[0],
                n2: model.normals[1],
            })
        );

        assert_eq!(t1, t0);
    }
}
