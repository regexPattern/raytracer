use serde::Deserialize;

use engine::{
    camera::Camera,
    matrix::{InvalidViewMatrix, Matrix},
    tuple::{Point, Vector},
};

use crate::tuple::{PointParser, VectorParser};

#[derive(Debug, PartialEq)]
pub enum InvalidCamera {
    EqualPositionAndLookingToPoints(Point),
    NullUpDirectionVector,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CameraParser {
    pub width: u32,
    pub height: u32,
    pub field_of_view: f64,
    pub position: PointParser,

    #[serde(default = "default_looking_at")]
    pub looking_at: PointParser,

    #[serde(default = "default_up_direction")]
    pub up_direction: VectorParser,
}

const fn default_looking_at() -> PointParser {
    PointParser {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }
}

const fn default_up_direction() -> VectorParser {
    VectorParser {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    }
}

impl From<InvalidViewMatrix> for InvalidCamera {
    fn from(ivm: InvalidViewMatrix) -> Self {
        match ivm {
            InvalidViewMatrix::EqualFromAndToPoints(point) => {
                Self::EqualPositionAndLookingToPoints(point)
            }
            InvalidViewMatrix::NullUpVector => Self::NullUpDirectionVector,
        }
    }
}

impl TryFrom<CameraParser> for Camera {
    type Error = InvalidCamera;

    fn try_from(c: CameraParser) -> Result<Self, Self::Error> {
        let from = Point::from(c.position);
        let to = Point::from(c.looking_at);
        let up = Vector::from(c.up_direction);

        let mut camera = Self::new(c.width, c.height, c.field_of_view.to_radians());
        camera.transform = Matrix::view(from, to, up)?;

        Ok(camera)
    }
}

impl std::error::Error for InvalidCamera {}

impl std::fmt::Display for InvalidCamera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EqualPositionAndLookingToPoints(point) => write!(
                f,
                "`position` and `looking_to` points must be different: `{{ x: {}, y: {}, z: {} }}`",
                point.0.x, point.0.y, point.0.z
            ),
            Self::NullUpDirectionVector => write!(f, "`up_direction` vector cannot be null"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_a_camera() {
        let input = r#"
{
    "width": 1920,
    "height": 1080,
    "field_of_view": 60,
    "position": {
        "x": 10,
        "y": 10,
        "z": 10
    },
    "looking_at": {
        "x": 12,
        "y": 13.5,
        "z": 1.25
    },
    "up_direction": {
        "x": 6.7,
        "y": 1,
        "z": 8
    }
}
        "#;

        let output: CameraParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            CameraParser {
                width: 1920,
                height: 1080,
                field_of_view: 60.0,
                position: PointParser {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                },
                looking_at: PointParser {
                    x: 12.0,
                    y: 13.5,
                    z: 1.25,
                },
                up_direction: VectorParser {
                    x: 6.7,
                    y: 1.0,
                    z: 8.0,
                }
            }
        );
    }

    #[test]
    fn the_default_looking_at_and_up_direction_for_a_camera() {
        let input = r#"
{
    "width": 1920,
    "height": 1080,
    "field_of_view": 60,
    "position": {
        "x": 10,
        "y": 10,
        "z": 10
    }
}
        "#;

        let output: CameraParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            CameraParser {
                width: 1920,
                height: 1080,
                field_of_view: 60.0,
                position: PointParser {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                },
                looking_at: PointParser {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                up_direction: VectorParser {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                }
            }
        );
    }

    #[test]
    fn getting_a_camera_from_a_parsed_camera() {
        let input = r#"
{
    "width": 1920,
    "height": 1080,
    "field_of_view": 60,
    "position": {
        "x": 10,
        "y": 10,
        "z": 10
    },
    "looking_at": {
        "x": 12,
        "y": 13.5,
        "z": 1.25
    },
    "up_direction": {
        "x": 6.7,
        "y": 1,
        "z": 8
    }
}
        "#;

        let output: CameraParser = serde_json::from_str(input).unwrap();

        let mut expected = Camera::new(1920, 1080, std::f64::consts::FRAC_PI_3);
        expected.transform = Matrix::view(
            Point::new(10.0, 10.0, 10.0),
            Point::new(12.0, 13.5, 1.25),
            Vector::new(6.7, 1.0, 8.0),
        )
        .unwrap();

        assert_eq!(Camera::try_from(output).unwrap(), expected);
    }

    #[test]
    fn parsing_a_camera_with_equal_position_and_looking_at_points() {
        let input = r#"
{
    "width": 1920,
    "height": 1080,
    "field_of_view": 60,
    "position": {
        "x": 10,
        "y": 10,
        "z": 10
    },
    "looking_at": {
        "x": 10,
        "y": 10,
        "z": 10
    }
}
        "#;

        let output: CameraParser = serde_json::from_str(input).unwrap();

        let expected = match Camera::try_from(output) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        };

        assert_eq!(
            expected,
            Err(
                "`position` and `looking_to` points must be different: `{ x: 10, y: 10, z: 10 }`"
                    .to_owned()
            )
        );
    }

    #[test]
    fn parsing_a_camera_with_null_up_direction_vector() {
        let input = r#"
{
    "width": 1920,
    "height": 1080,
    "field_of_view": 60,
    "position": {
        "x": 10,
        "y": 10,
        "z": 10
    },
    "up_direction": {
        "x": 0,
        "y": 0,
        "z": 0
    }
}
        "#;

        let output: CameraParser = serde_json::from_str(input).unwrap();

        let expected = match Camera::try_from(output) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        };

        assert_eq!(
            expected,
            Err("`up_direction` vector cannot be null".to_owned())
        );
    }
}
