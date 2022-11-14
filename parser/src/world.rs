use serde::Deserialize;

use core::{light::PointLight, shape::Shape, world::World};

use super::{light::PointLightParser, shape::ShapeParser};

#[derive(Debug, Deserialize, Default, PartialEq)]
#[serde(default)]
struct WorldParser {
    objects: Vec<ShapeParser>,
    lights: Vec<PointLightParser>,
}

impl From<WorldParser> for World {
    fn from(w: WorldParser) -> Self {
        let objects = w.objects.into_iter().map(|s| Shape::from(s)).collect();
        let lights = w.lights.into_iter().map(|l| PointLight::from(l)).collect();

        Self { objects, lights }
    }
}

#[cfg(test)]
mod tests {
    use core::{
        color::Color,
        shape::{Figure, Plane, Sphere},
        tuple::Point,
    };

    use crate::{color::ColorParser, shape::FigureParser, tuple::PointParser};

    use super::*;

    #[test]
    fn the_default_world_has_no_objects_and_no_lights() {
        let input = r#"
{}
        "#;

        let output: WorldParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            WorldParser {
                objects: vec![],
                lights: vec![],
            }
        );
    }

    #[test]
    fn parsing_a_world() {
        let input = r#"
{
    "objects": [
        {
            "type": "sphere"
        },
        {
            "type": "plane"
        }
    ],
    "lights": [
        {
            "position": {
                "x": 10,
                "y": 5.5,
                "z": 0
            },
            "intensity": {
                "red": 255,
                "green": 127,
                "blue": 99
            }
        }
    ]
}
        "#;

        let output: WorldParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            WorldParser {
                objects: vec![
                    ShapeParser::Sphere(FigureParser::default()),
                    ShapeParser::Plane(FigureParser::default())
                ],
                lights: vec![PointLightParser {
                    position: PointParser {
                        x: 10.0,
                        y: 5.5,
                        z: 0.0,
                    },
                    intensity: ColorParser {
                        red: 255,
                        green: 127,
                        blue: 99,
                    }
                }],
            }
        );
    }

    #[test]
    fn getting_a_world_from_a_parsed_world() {
        let input = r#"
{
    "objects": [
        {
            "type": "sphere"
        },
        {
            "type": "plane"
        }
    ],
    "lights": [
        {
            "position": {
                "x": 10,
                "y": 5.5,
                "z": 0
            },
            "intensity": {
                "red": 255,
                "green": 0,
                "blue": 0
            }
        }
    ]
}
        "#;

        let output: WorldParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            World::from(output),
            World {
                objects: vec![
                    Shape::Sphere(Sphere(Figure::default())),
                    Shape::Plane(Plane(Figure::default()))
                ],
                lights: vec![PointLight {
                    position: Point::new(10.0, 5.5, 0.0),
                    intensity: Color {
                        red: 1.0,
                        green: 0.0,
                        blue: 0.0,
                    }
                }],
            }
        );
    }
}
