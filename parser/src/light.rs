use serde::Deserialize;

use engine::{color::Color, light::PointLight, tuple::Point};

use crate::{color::ColorParser, tuple::PointParser};

#[derive(Debug, Deserialize, PartialEq)]
pub struct PointLightParser {
    pub position: PointParser,
    pub intensity: ColorParser,
}

impl From<PointLightParser> for PointLight {
    fn from(l: PointLightParser) -> Self {
        let position = Point::from(l.position);
        let intensity = Color::from(l.intensity);

        Self {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use engine::color;

    use super::*;

    #[test]
    fn parsing_a_light() {
        let input = r#"
{
    "position": {
        "x": 10,
        "y": 10,
        "z": 10
    },
    "intensity": {
        "red": 255,
        "green": 0,
        "blue": 0
    }
}
        "#;

        let output: PointLightParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            PointLightParser {
                position: PointParser {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                },
                intensity: ColorParser {
                    red: 255,
                    green: 0,
                    blue: 0
                }
            }
        );
    }

    #[test]
    fn getting_a_light_from_a_parsed_light() {
        let input = r#"
{
    "position": {
        "x": 10,
        "y": 10,
        "z": 10
    },
    "intensity": {
        "red": 255,
        "green": 0,
        "blue": 0
    }
}
        "#;

        let output: PointLightParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            PointLight::from(output),
            PointLight {
                position: Point::new(10.0, 10.0, 10.0),
                intensity: color::RED
            }
        );
    }
}
