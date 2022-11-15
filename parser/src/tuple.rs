use serde::Deserialize;

use engine::tuple::{Point, Vector};

#[derive(Debug, Deserialize, PartialEq)]
pub struct PointParser {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct VectorParser {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<PointParser> for Point {
    fn from(p: PointParser) -> Self {
        Self::new(p.x, p.y, p.z)
    }
}

impl From<VectorParser> for Vector {
    fn from(v: VectorParser) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_a_point() {
        let input = r#"
{
    "x": 10,
    "y": 11,
    "z": 12
}
        "#;

        let output: PointParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            PointParser {
                x: 10.0,
                y: 11.0,
                z: 12.0,
            }
        );
    }

    #[test]
    fn parsing_a_vector() {
        let input = r#"
{
    "x": 10,
    "y": 11,
    "z": 12
}
        "#;

        let output: VectorParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            VectorParser {
                x: 10.0,
                y: 11.0,
                z: 12.0,
            }
        );
    }

    #[test]
    fn getting_a_point_from_a_parsed_point() {
        let input = r#"
{
    "x": 10,
    "y": 11,
    "z": 12
}
        "#;

        let output: PointParser = serde_json::from_str(input).unwrap();

        assert_eq!(Point::from(output), Point::new(10.0, 11.0, 12.0));
    }

    #[test]
    fn getting_a_vector_from_a_parsed_vector() {
        let input = r#"
{
    "x": 10,
    "y": 11,
    "z": 12
}
        "#;

        let output: VectorParser = serde_json::from_str(input).unwrap();

        assert_eq!(Vector::from(output), Vector::new(10.0, 11.0, 12.0));
    }
}
