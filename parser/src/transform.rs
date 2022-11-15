use serde::Deserialize;

use engine::matrix::{self, Matrix};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum TransformParser {
    Identity,
    RotationX {
        degrees: f64,
    },
    RotationY {
        degrees: f64,
    },
    RotationZ {
        degrees: f64,
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
            TransformParser::RotationX { degrees } => Self::rotation_x(degrees.to_radians()),
            TransformParser::RotationY { degrees } => Self::rotation_y(degrees.to_radians()),
            TransformParser::RotationZ { degrees } => Self::rotation_z(degrees.to_radians()),
            TransformParser::Scaling { x, y, z } => Self::scaling(x, y, z),
            TransformParser::Shearing {
                xy,
                xz,
                yx,
                yz,
                zx,
                zy,
            } => Self::shearing(xy, xz, yx, yz, zx, zy),
            TransformParser::Translation { x, y, z } => Self::translation(x, y, z),
        }
    }
}

impl From<MultipleTransformParser> for Matrix<4, 4> {
    fn from(mt: MultipleTransformParser) -> Self {
        mt.0.into_iter()
            .fold(matrix::IDENTITY4X4, |acc, t| Self::from(t) * acc)
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
    "degrees": 2
}
        "#;

        let output: TransformParser = serde_json::from_str(input).unwrap();

        assert_eq!(output, TransformParser::RotationX { degrees: 2.0 });
    }

    #[test]
    fn parsing_a_rotation_y_transformation() {
        let input = r#"
{
    "type": "rotation_y",
    "degrees": 1.5
}
        "#;

        let output: TransformParser = serde_json::from_str(input).unwrap();

        assert_eq!(output, TransformParser::RotationY { degrees: 1.5 });
    }

    #[test]
    fn parsing_a_rotation_z_transformation() {
        let input = r#"
{
    "type": "rotation_z",
    "degrees": 1
}
        "#;

        let output: TransformParser = serde_json::from_str(input).unwrap();

        assert_eq!(output, TransformParser::RotationZ { degrees: 1.0 });
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
