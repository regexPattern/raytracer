use std::num::NonZeroUsize;

use thiserror::Error;

use crate::{
    shape::{Group, GroupBuilder, Shape, SmoothTriangle, Triangle, TriangleBuilder},
    transform::Transform,
    tuple::{Point, Vector},
};

/// Minimum number of vertices required to create a polygon.
pub const MIN_POLYGON_VERTICES: usize = 3;

/// The error type when trying to parse a model.
///
/// Errors originate from the model spec format itself.
///
#[derive(Clone, Debug, Error, PartialEq)]
#[error("parsing error at line {}: '{kind}'", line_nr + 1)]
pub struct Error {
    /// Kind of the parsing error.
    pub kind: ErrorKind,

    /// Line where the error was found.
    pub line_nr: usize,
}

/// Enum to store the various types of errors that can happen when parsing a model.
#[derive(Clone, Debug, Error, PartialEq)]
pub enum ErrorKind {
    /// A value in some of a vertex's coordinates declaration could not be parsed as a floating
    /// point number.
    #[error(transparent)]
    InvalidCoordinate(#[from] std::num::ParseFloatError),

    /// A vertex index in a face declaration could not be parsed a non-zero positive integer.
    #[error(transparent)]
    InvalidVertexIndex(#[from] std::num::ParseIntError),

    /// The face declaration has less than the required amount of vertices necessary to create a
    /// valid polygon. This value is set to [MIN_POLYGON_VERTICES].
    #[error("insufficient vertices for a polygon")]
    InsufficientVertices,

    /// The accessed vertex index in a face declaration refers to the index of a vertex that hasn't
    /// been previously declared.
    #[error("no element at index: `{accessed}` out of `{available}` available (1-indexed)")]
    FaceElementOutOfBounds {
        accessed: NonZeroUsize,
        available: usize,
    },

    /// The vertex declaration doesn't have the specified component.
    #[error("missing field: `{name}`")]
    MissingField { name: &'static str },
}

/// In-memory Representation of a 3D model
///
/// At the time being this only supports models exported in [WaveFront OBJ
/// format](https://en.wikipedia.org/wiki/Wavefront_.obj_file), but maybe more formats are going to
/// be added in the future.
///
/// Keep in mind that models get loaded the program runs, there's no caching of previously loaded
/// models, which can be a performance drawback for really large models. This is a possible future
/// optimization.
///
/// # Examples
///
/// A model must be built from an [OBJModelBuilder].
///
/// ```no_run
/// use raytracer::{
///     shape::Group,
///     model::{Model, OBJModelBuilder},
///     transform::Transform,
/// };
///
/// let model_spec = std::fs::read_to_string("filename.obj").unwrap();
///
/// let model = Model::try_from(OBJModelBuilder {
///     model_spec: &model_spec,
///     transform: Transform::scaling(2.0, 2.0, 2.0).unwrap(),
/// }).unwrap();
///
/// // Models are only useful when converted to a `Shape::Group`,
/// // which can later be added to a world.
/// let group = Group::from(model);
///
/// ```
///
#[derive(Debug, PartialEq)]
pub struct Model {
    groups: Vec<PolygonsGroup>,
    normals: Vec<Vector>,
    vertices: Vec<Point>,
    transform: Transform,
}

/// Builder for a model exported in [WaveFront OBJ
/// Format](https://en.wikipedia.org/wiki/Wavefront_.obj_file).
#[derive(Clone)]
pub struct OBJModelBuilder<'a> {
    /// Reference to a string with a model represented in WaveFront OBJ format.
    pub model_spec: &'a str,

    /// Transformation that's going to be applied to the model once it's converted to a
    /// [Group](crate::shape::Group).
    pub transform: Transform,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct FaceVertex {
    vertex: Point,
    normal: Option<Vector>,
}

#[derive(Debug, PartialEq)]
struct PolygonsGroup {
    group: Group,
    name: String,
}

impl TryFrom<OBJModelBuilder<'_>> for Model {
    type Error = Error;

    fn try_from(builder: OBJModelBuilder) -> Result<Self, Self::Error> {
        let OBJModelBuilder {
            model_spec: content,
            transform,
        } = builder;

        let mut groups = vec![PolygonsGroup {
            group: Group::default(),
            name: "__default".to_string(),
        }];

        let mut normals = vec![];
        let mut vertices = vec![];

        for (line_nr, line) in content.lines().enumerate() {
            let propagate_line_err = |kind| Error { kind, line_nr };
            let mut fields = line.split_whitespace();

            let data_type = fields.next();
            let data = fields.fuse();

            match data_type {
                Some("v") => {
                    let (x, y, z) = Self::parse_coordinate(data).map_err(propagate_line_err)?;
                    vertices.push(Point::new(x, y, z));
                }
                Some("vn") => {
                    let (x, y, z) = Self::parse_coordinate(data).map_err(propagate_line_err)?;
                    normals.push(Vector::new(x, y, z));
                }
                Some("f") => {
                    let face =
                        Self::parse_face(data, &normals, &vertices).map_err(propagate_line_err)?;

                    // There's always going to be a valid group in the group's queue, as it always
                    // contains at least the "__default" group.
                    #[allow(clippy::unwrap_used)]
                    groups.last_mut().unwrap().group.extend(face);
                }
                Some("g") => {
                    groups.push(Self::parse_group(data).map_err(propagate_line_err)?);
                }
                _ => (),
            }
        }

        Ok(Model {
            groups,
            normals,
            vertices,
            transform,
        })
    }
}

impl From<Model> for Group {
    fn from(model: Model) -> Self {
        let group_builder = GroupBuilder {
            children: model
                .groups
                .into_iter()
                .map(|polygons_group| Shape::Group(polygons_group.group)),
            transform: model.transform,
        };

        Self::from(group_builder)
    }
}

impl TryFrom<OBJModelBuilder<'_>> for Group {
    type Error = Error;

    fn try_from(builder: OBJModelBuilder<'_>) -> Result<Self, Self::Error> {
        let model = Model::try_from(builder)?;
        Ok(Group::from(model))
    }
}

impl Model {
    fn parse_coordinate<'a, T>(mut data: T) -> Result<(f64, f64, f64), ErrorKind>
    where
        T: Iterator<Item = &'a str>,
    {
        let x = data
            .next()
            .ok_or(ErrorKind::MissingField { name: "x" })?
            .parse::<f64>()?;

        let y = data
            .next()
            .ok_or(ErrorKind::MissingField { name: "y" })?
            .parse::<f64>()?;

        let z = data
            .next()
            .ok_or(ErrorKind::MissingField { name: "z" })?
            .parse::<f64>()?;

        Ok((x, y, z))
    }

    fn parse_face<'a, T>(
        data: T,
        saved_normals: &[Vector],
        saved_vertices: &[Point],
    ) -> Result<Vec<Shape>, ErrorKind>
    where
        T: Iterator<Item = &'a str>,
    {
        let elements: Vec<_> = data.collect();

        if elements.len() < MIN_POLYGON_VERTICES {
            return Err(ErrorKind::InsufficientVertices);
        }

        let mut vertices = vec![];

        for elem in elements {
            let mut fields = elem.split('/');

            // There's always going to be an element in the split's first position. This element might
            // be empty, but it is there. Empty elements are going to be handled when parsing them into
            // numbers from `get_face_element()`.
            #[allow(clippy::unwrap_used)]
            let vertex = Self::get_face_element(fields.next().unwrap(), saved_vertices)?;

            fields.next();

            let normal = match fields.next() {
                Some(normal_index) => Some(Self::get_face_element(normal_index, saved_normals)?),
                None => None,
            };

            vertices.push(FaceVertex { vertex, normal });
        }

        Self::fan_triangulation(vertices)
    }

    fn get_face_element<T>(raw: &str, saved_elements: &[T]) -> Result<T, ErrorKind>
    where
        T: Copy,
    {
        let index = raw.parse::<NonZeroUsize>()?;
        saved_elements
            .get(index.get() - 1)
            .ok_or(ErrorKind::FaceElementOutOfBounds {
                accessed: index,
                available: saved_elements.len(),
            })
            .copied()
    }

    fn fan_triangulation(vertices: Vec<FaceVertex>) -> Result<Vec<Shape>, ErrorKind> {
        let mut triangles = vec![];

        for i in 2..vertices.len() {
            let v0 = vertices[0];
            let v1 = vertices[i - 1];
            let v2 = vertices[i];

            // I've noticed that some OBJ files generate polygons that cannot be decomposed exactly
            // as triangles, because some of their vertices end up creating triangles with
            // collinear sides. This doesn't happen often, so I just ignore those triangles when
            // they are generated.
            if let Ok(triangle) = Triangle::try_from(TriangleBuilder {
                material: Default::default(),
                vertices: [v0.vertex, v1.vertex, v2.vertex],
            }) {
                let triangle =
                    if let (Some(n0), Some(n1), Some(n2)) = (v0.normal, v1.normal, v2.normal) {
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

    fn parse_group<'a, T>(mut data: T) -> Result<PolygonsGroup, ErrorKind>
    where
        T: Iterator<Item = &'a str>,
    {
        let group_name = data
            .next()
            .ok_or(ErrorKind::MissingField { name: "group_name" })?;

        Ok(PolygonsGroup {
            group: Group::default(),
            name: group_name.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::shape::TriangleBuilder;

    use super::*;

    #[test]
    fn parsing_vertex_records() {
        let input = "\
v -1 1 0
v -1.0000 0.50000 0.0000
v 1 0 0
v 1 1 0";

        let model = Model::try_from(OBJModelBuilder {
            model_spec: input,
            transform: Default::default(),
        })
        .unwrap();

        assert_eq!(model.vertices[0], Point::new(-1.0, 1.0, 0.0));
        assert_eq!(model.vertices[1], Point::new(-1.0, 0.5, 0.0));
        assert_eq!(model.vertices[2], Point::new(1.0, 0.0, 0.0));
        assert_eq!(model.vertices[3], Point::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn parsing_a_vertex() {
        let input = "1 2.5000 -3.0".split_whitespace();

        let vertex = Model::parse_coordinate(input).unwrap();

        assert_eq!(vertex, (1.0, 2.5, -3.0));
    }

    #[test]
    fn trying_to_parse_a_vertex_with_a_missing_field() {
        assert_eq!(
            Model::parse_coordinate("".split_whitespace()),
            Err(ErrorKind::MissingField { name: "x" })
        );

        assert_eq!(
            Model::parse_coordinate("1".split_whitespace()),
            Err(ErrorKind::MissingField { name: "y" })
        );

        assert_eq!(
            Model::parse_coordinate("1 2.5".split_whitespace()),
            Err(ErrorKind::MissingField { name: "z" })
        );
    }

    #[test]
    fn trying_to_parse_a_vertex_with_an_invalid_coordinate() {
        assert!(matches!(
            Model::parse_coordinate("1 @ 2.0".split_whitespace()),
            Err(ErrorKind::InvalidCoordinate(_))
        ));
    }

    #[test]
    fn getting_error_with_line_information_when_parsing_fails() {
        let input = "v 1";

        assert_eq!(
            Model::try_from(OBJModelBuilder {
                model_spec: input,
                transform: Default::default()
            }),
            Err(Error {
                kind: ErrorKind::MissingField { name: "y" },
                line_nr: 0,
            })
        );
    }

    #[test]
    fn an_error_should_display_with_correct_message() {
        let input = "v 1";

        let err = Model::try_from(OBJModelBuilder {
            model_spec: input,
            transform: Default::default(),
        })
        .unwrap_err();

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

        let model = Model::try_from(OBJModelBuilder {
            model_spec: input,
            transform: Default::default(),
        })
        .unwrap();

        let g = &model.groups[0].group;
        let t0 = &g.children[0];
        let t1 = &g.children[1];

        assert_eq!(
            t0,
            &Shape::Triangle(
                Triangle::try_from(TriangleBuilder {
                    material: Default::default(),
                    vertices: [model.vertices[0], model.vertices[1], model.vertices[2]]
                })
                .unwrap()
            )
        );

        assert_eq!(
            t1,
            &Shape::Triangle(
                Triangle::try_from(TriangleBuilder {
                    material: Default::default(),
                    vertices: [model.vertices[0], model.vertices[2], model.vertices[3]]
                })
                .unwrap()
            )
        );
    }

    #[test]
    fn trying_to_parse_a_face_with_insufficient_vertices() {
        let input = "f ".split_whitespace();

        let err = Model::parse_face(input, &[], &[]).unwrap_err();

        assert_eq!(err, ErrorKind::InsufficientVertices);
    }

    #[test]
    fn trying_to_parse_a_face_element_with_an_invalid_vertex_index() {
        assert!(matches!(
            Model::get_face_element::<Point>("@", &[]),
            Err(ErrorKind::InvalidVertexIndex(_))
        ));

        let err = Model::get_face_element("2", &[Point::new(1.0, 2.0, 3.0)]).unwrap_err();

        assert_eq!(
            err,
            ErrorKind::FaceElementOutOfBounds {
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

        let vertex = Model::get_face_element("3", &vertices).unwrap();

        assert_eq!(vertex, vertices[2]);
    }

    #[test]
    fn parsing_a_single_triangle_face() {
        let vertices = [
            Point::new(2.0, 5.0, 1.0),
            Point::new(7.0, -2.0, 3.0),
            Point::new(4.0, 1.5, 4.25),
        ];

        let input = "1 2 3".split_whitespace();

        let tri = Model::parse_face(input, &[], &vertices).unwrap();

        assert_eq!(
            tri[0],
            Shape::Triangle(
                Triangle::try_from(TriangleBuilder {
                    material: Default::default(),
                    vertices
                })
                .unwrap()
            )
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

        let model = Model::try_from(OBJModelBuilder {
            model_spec: input,
            transform: Default::default(),
        })
        .unwrap();

        let g = &model.groups[0].group;
        let t0 = &g.children[0];
        let t1 = &g.children[1];
        let t2 = &g.children[2];

        assert_eq!(
            t0,
            &Shape::Triangle(
                Triangle::try_from(TriangleBuilder {
                    material: Default::default(),
                    vertices: [model.vertices[0], model.vertices[1], model.vertices[2]]
                })
                .unwrap()
            )
        );

        assert_eq!(
            t1,
            &Shape::Triangle(
                Triangle::try_from(TriangleBuilder {
                    material: Default::default(),
                    vertices: [model.vertices[0], model.vertices[2], model.vertices[3]]
                })
                .unwrap()
            )
        );

        assert_eq!(
            t2,
            &Shape::Triangle(
                Triangle::try_from(TriangleBuilder {
                    material: Default::default(),
                    vertices: [model.vertices[0], model.vertices[3], model.vertices[4]]
                })
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

        let model = Model::try_from(OBJModelBuilder {
            model_spec: input,
            transform: Default::default(),
        })
        .unwrap();

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
                Triangle::try_from(TriangleBuilder {
                    material: Default::default(),
                    vertices: [model.vertices[0], model.vertices[1], model.vertices[2]]
                })
                .unwrap()
            )
        );

        assert_eq!(
            t1,
            &Shape::Triangle(
                Triangle::try_from(TriangleBuilder {
                    material: Default::default(),
                    vertices: [model.vertices[0], model.vertices[2], model.vertices[3]]
                })
                .unwrap()
            )
        );
    }

    #[test]
    fn trying_to_parse_a_group_without_a_name() {
        assert_eq!(
            Model::parse_group("".split_whitespace()),
            Err(ErrorKind::MissingField { name: "group_name" })
        );
    }

    #[test]
    fn parsing_vertex_normal_records() {
        let input = r"\
vn 0 0 1
vn 0.707 0 -0.707
vn 1 2 3";

        let model = Model::try_from(OBJModelBuilder {
            model_spec: input,
            transform: Default::default(),
        })
        .unwrap();

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

        let model = Model::try_from(OBJModelBuilder {
            model_spec: input,
            transform: Default::default(),
        })
        .unwrap();

        let g = &model.groups[0].group;
        let t0 = &g.children[0];
        let t1 = &g.children[1];

        assert_eq!(
            t0,
            &Shape::SmoothTriangle(SmoothTriangle {
                triangle: Triangle::try_from(TriangleBuilder {
                    material: Default::default(),
                    vertices: [model.vertices[0], model.vertices[1], model.vertices[2]]
                })
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

        let input = "1//3 2//2 3//1".split_whitespace();

        let tri = Model::parse_face(input, &normals, &vertices).unwrap();

        assert_eq!(
            tri[0],
            Shape::SmoothTriangle(SmoothTriangle {
                triangle: Triangle::try_from(TriangleBuilder {
                    material: Default::default(),
                    vertices
                })
                .unwrap(),
                n0: normals[2],
                n1: normals[1],
                n2: normals[0]
            })
        );
    }
}
