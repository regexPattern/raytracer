use serde::Deserialize;

use crate::matrix::{self, Matrix};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum TransformParser {
    Identity,
    RotationX {
        radians: f64,
    },
    RotationY {
        radians: f64,
    },
    RotationZ {
        radians: f64,
    },
    Scaling {
        x: f64,
        y: f64,
        z: f64,
    },
    Shearing {
        xy: f64,
        xz: f64,
        yx: f64,
        yz: f64,
        zx: f64,
        zy: f64,
    },
    Translation {
        x: f64,
        y: f64,
        z: f64,
    },
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct MultipleTransformParser(pub Vec<TransformParser>);

impl Default for TransformParser {
    fn default() -> Self {
        Self::Identity
    }
}

impl Default for MultipleTransformParser {
    fn default() -> Self {
        Self(vec![TransformParser::Identity])
    }
}

impl From<TransformParser> for Matrix<4, 4> {
    fn from(t: TransformParser) -> Self {
        match t {
            TransformParser::Identity => matrix::IDENTITY4X4,
            TransformParser::RotationX { radians } => Matrix::rotation_x(radians),
            TransformParser::RotationY { radians } => Matrix::rotation_y(radians),
            TransformParser::RotationZ { radians } => Matrix::rotation_z(radians),
            TransformParser::Scaling { x, y, z } => Matrix::scaling(x, y, z),
            TransformParser::Shearing {
                xy,
                xz,
                yx,
                yz,
                zx,
                zy,
            } => Matrix::shearing(xy, xz, yx, yz, zx, zy),
            TransformParser::Translation { x, y, z } => Matrix::translation(x, y, z),
        }
    }
}

impl From<MultipleTransformParser> for Matrix<4, 4> {
    fn from(mt: MultipleTransformParser) -> Matrix<4, 4> {
        mt.0.into_iter()
            .fold(matrix::IDENTITY4X4, |acc, t| Matrix::from(t) * acc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_a_rotation_x_transformation() {
        let input = r#"
{
    "type": "rotation_x",
    "radians": 2
}
        "#;

        let output: TransformParser = serde_json::from_str(input).unwrap();

        assert_eq!(output, TransformParser::RotationX { radians: 2.0 });
    }

    #[test]
    fn parsing_a_rotation_y_transformation() {
        let input = r#"
{
    "type": "rotation_y",
    "radians": 1.5
}
        "#;

        let output: TransformParser = serde_json::from_str(input).unwrap();

        assert_eq!(output, TransformParser::RotationY { radians: 1.5 });
    }

    #[test]
    fn parsing_a_rotation_z_transformation() {
        let input = r#"
{
    "type": "rotation_z",
    "radians": 1
}
        "#;

        let output: TransformParser = serde_json::from_str(input).unwrap();

        assert_eq!(output, TransformParser::RotationZ { radians: 1.0 });
    }

    #[test]
    fn parsing_a_scaling_transformation() {
        let input = r#"
{
    "type": "scaling",
    "x": 1.1,
    "y": 2.2,
    "z": 3.3
}
        "#;

        let output: TransformParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            TransformParser::Scaling {
                x: 1.1,
                y: 2.2,
                z: 3.3
            }
        );
    }

    #[test]
    fn parsing_a_shearing_transformation() {
        let input = r#"
{
    "type": "shearing",
    "xy": 1,
    "xz": 1.1,
    "yx": 2,
    "yz": 2.1,
    "zx": 3,
    "zy": 3.1
}
        "#;

        let output: TransformParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            TransformParser::Shearing {
                xy: 1.0,
                xz: 1.1,
                yx: 2.0,
                yz: 2.1,
                zx: 3.0,
                zy: 3.1,
            }
        )
    }

    #[test]
    fn parsing_a_translation_transformation() {
        let input = r#"
{
    "type": "translation",
    "x": 1.1,
    "y": 1.2,
    "z": 1.3
}
        "#;

        let output: TransformParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            TransformParser::Translation {
                x: 1.1,
                y: 1.2,
                z: 1.3
            }
        );
    }

    #[test]
    fn getting_a_matrix_from_a_parsed_transformation() {
        let input = r#"
{
    "type": "translation",
    "x": 1.1,
    "y": 1.2,
    "z": 1.3
}
        "#;

        let output: TransformParser = serde_json::from_str(input).unwrap();

        assert_eq!(Matrix::from(output), Matrix::translation(1.1, 1.2, 1.3));
    }

    #[test]
    fn parsing_multiple_transformations() {
        let input = r#"
[
    {
        "type": "translation",
        "x": 1,
        "y": 3,
        "z": 5
    },
    {
        "type": "scaling",
        "x": 2,
        "y": 4,
        "z": 6
    }
]
        "#;

        let output: MultipleTransformParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            MultipleTransformParser(vec![
                TransformParser::Translation {
                    x: 1.0,
                    y: 3.0,
                    z: 5.0
                },
                TransformParser::Scaling {
                    x: 2.0,
                    y: 4.0,
                    z: 6.0
                }
            ])
        );

        assert_eq!(
            Matrix::from(output),
            Matrix::scaling(2.0, 4.0, 6.0) * Matrix::translation(1.0, 3.0, 5.0)
        )
    }

    #[test]
    fn the_default_transformation_when_there_are_multiple_transformations() {
        let input = r#"
[]
        "#;

        let output: MultipleTransformParser = serde_json::from_str(input).unwrap();

        assert_eq!(Matrix::from(output), matrix::IDENTITY4X4);
    }
}
