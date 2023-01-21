use std::num::NonZeroUsize;

use crate::{
    shape::{CollinearTriangleSidesError, Group, Shape, Triangle},
    tuple::Point,
};

#[derive(Debug, PartialEq)]
pub struct LineInfo<'a> {
    number: usize,
    data: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct ParsingError<'a> {
    kind: ParsingErrorKind,
    line: LineInfo<'a>,
}

#[derive(Debug, PartialEq)]
pub enum ParsingErrorKind {
    InsufficientVertices,
    InvalidPolygon,
    InvalidValue,
    MissingValue,
}

#[derive(Debug, PartialEq)]
pub struct Parser {
    pub group: Group,
    vertices: Vec<Point>,
}

impl From<CollinearTriangleSidesError> for ParsingErrorKind {
    fn from(value: CollinearTriangleSidesError) -> Self {
        Self::InvalidPolygon
    }
}

fn parse_value<T>(data: Option<&str>) -> Result<T, ParsingErrorKind>
where
    T: std::str::FromStr,
{
    Ok(data
        .ok_or(ParsingErrorKind::MissingValue)?
        .parse::<T>()
        .map_err(|_| ParsingErrorKind::InvalidValue)?)
}

fn parse_point(raw: &str) -> Result<Point, ParsingErrorKind> {
    let mut split = raw.split(" ").skip(1);

    let x = parse_value::<f64>(split.next())?;
    let y = parse_value::<f64>(split.next())?;
    let z = parse_value::<f64>(split.next())?;

    Ok(Point::new(x, y, z))
}

fn parse_polygon(
    data: &str,
    defined_vertices: &[Point],
) -> Result<Vec<Triangle>, ParsingErrorKind> {
    if defined_vertices.len() < 3 {
        return Err(ParsingErrorKind::InsufficientVertices);
    }

    let mut polygon_vertices = vec![];
    let fields = data.split(" ").skip(1);

    for field in fields {
        let index = parse_value::<NonZeroUsize>(Some(field))?.get() - 1;
        if let Some(vertex) = defined_vertices.get(index).copied() {
            polygon_vertices.push(vertex);
        }
    }

    Ok(fan_triangulation(&polygon_vertices)?)
}

fn fan_triangulation(vertices: &[Point]) -> Result<Vec<Triangle>, ParsingErrorKind> {
    let mut triangles = vec![];

    for i in 2..vertices.len() {
        let triangle = Triangle::try_new(vertices[0], vertices[i - 1], vertices[i])
            .map_err(|_| ParsingErrorKind::InvalidPolygon)?;

        triangles.push(triangle);
    }

    Ok(triangles)
}

impl Parser {
    pub fn parse(content: &str) -> Result<Self, ParsingError> {
        let mut triangles = vec![];
        let mut vertices = vec![];

        for (line, data) in content.lines().enumerate() {
            if data.starts_with("v ") {
                vertices.push(parse_point(data).map_err(|kind| ParsingError {
                    kind,
                    line: LineInfo {
                        number: line + 1,
                        data,
                    },
                })?);
            } else if data.starts_with("f ") {
                let polygons = parse_polygon(data, &vertices)
                    .map_err(|kind| ParsingError {
                        kind,
                        line: LineInfo {
                            number: line + 1,
                            data,
                        },
                    })?
                    .into_iter()
                    .map(|triangle| Shape::Triangle(triangle));

                triangles.extend(polygons);
            }
        }

        Ok(Self {
            group: Group::new(triangles, Default::default()),
            vertices,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;

    use super::*;

    #[test]
    fn ignoring_unrecognized_lines() {
        let input = "\
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";

        let parser = Parser::parse(&input);

        // todo!();
    }

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
    fn parsing_a_point() {
        let point = super::parse_point("v 10.5 -1 0").unwrap();
        let invalid_value = super::parse_point("v 1 @ 3");
        let missing_value = super::parse_point("v 1 2");

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

        let parser = Parser::parse(&input).unwrap();

        assert_eq!(parser.vertices[0], Point::new(-1.0, 1.0, 0.0));
        assert_eq!(parser.vertices[1], Point::new(-1.0, 0.5, 0.0));
        assert_eq!(parser.vertices[2], Point::new(1.0, 0.0, 0.0));
        assert_eq!(parser.vertices[3], Point::new(1.0, 1.0, 0.0));
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

        let polygon = super::parse_polygon("f 1 2 3", &defined_vertices).unwrap();

        assert_eq!(
            polygon[0],
            Triangle::try_new(
                defined_vertices[0],
                defined_vertices[1],
                defined_vertices[2]
            )
            .unwrap()
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

        let parser = Parser::parse(&input).unwrap();

        let g = parser.group;

        let t0 = &g.children[0];
        let t1 = &g.children[1];

        let expected_t0 = Shape::Triangle(
            Triangle::try_new(parser.vertices[0], parser.vertices[1], parser.vertices[2]).unwrap(),
        );

        let expected_t1 = Shape::Triangle(
            Triangle::try_new(parser.vertices[0], parser.vertices[2], parser.vertices[3]).unwrap(),
        );

        assert!(matches!(t0, expected_t0));
        assert!(matches!(t1, expected_t1));
    }

    #[test]
    fn trying_to_parse_an_invalid_polygon() {
        let defined_vertices = [
            Point::new(1.0, 1.0, 1.0),
            Point::new(2.0, 2.0, 2.0),
            Point::new(3.0, 3.0, 3.0),
        ];

        assert_eq!(
            super::parse_polygon("f 1 2 3", &defined_vertices),
            Err(ParsingErrorKind::InvalidPolygon)
        );
    }

    #[test]
    fn trying_to_parse_a_polygon_with_less_than_3_vertices_defined() {
        let defined_vertices = [Point::new(1.0, 1.0, 1.0), Point::new(2.0, 2.0, 2.0)];

        assert_eq!(
            super::parse_polygon("f 1 2 3", &defined_vertices),
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

        let parser = Parser::parse(&input).unwrap();

        let g = parser.group;

        let t0 = &g.children[0];
        let t1 = &g.children[1];
        let t2 = &g.children[2];

        let expected_t0 = Shape::Triangle(
            Triangle::try_new(parser.vertices[0], parser.vertices[1], parser.vertices[2]).unwrap(),
        );

        let expected_t1 = Shape::Triangle(
            Triangle::try_new(parser.vertices[0], parser.vertices[2], parser.vertices[3]).unwrap(),
        );

        let expected_t2 = Shape::Triangle(
            Triangle::try_new(parser.vertices[0], parser.vertices[3], parser.vertices[4]).unwrap(),
        );
    }

    #[test]
    fn converting_an_obj_file_to_a_group() {
        let input = r"\
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

f 3 2 1
f 1 2 4";

        let t0 = Shape::Triangle(
            Triangle::try_new(
                Point::new(1.0, 0.0, 0.0),
                Point::new(-1.0, 0.0, 0.0),
                Point::new(-1.0, 1.0, 0.0),
            )
            .unwrap(),
        );

        let t1 = Shape::Triangle(
            Triangle::try_new(
                Point::new(-1.0, 1.0, 0.0),
                Point::new(-1.0, 0.0, 0.0),
                Point::new(1.0, 1.0, 0.0),
            )
            .unwrap(),
        );

        let g = Group::new([t0, t1], Default::default());

        assert_eq!(g, Parser::parse(&input).unwrap().group);
    }
}
