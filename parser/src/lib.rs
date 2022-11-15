#![allow(clippy::module_name_repetitions)]

mod camera;
mod color;
mod light;
mod material;
mod pattern;
mod shape;
mod texture;
mod transform;
mod tuple;
mod world;

use serde::Deserialize;

use engine::{camera::Camera, canvas::Canvas, world::World};

use camera::{CameraParser, InvalidCamera};
use world::WorldParser;

#[derive(Debug)]
pub enum InvalidScene {
    InvalidCamera(InvalidCamera),
    ParsingError(serde_json::Error),
}

#[derive(Debug, PartialEq)]
pub struct Scene {
    pub camera: Camera,
    pub world: World,
}

#[derive(Debug, Deserialize, PartialEq)]
struct SceneParser {
    camera: CameraParser,
    world: WorldParser,
}

impl From<serde_json::Error> for InvalidScene {
    fn from(err: serde_json::Error) -> Self {
        Self::ParsingError(err)
    }
}

impl From<InvalidCamera> for InvalidScene {
    fn from(err: InvalidCamera) -> Self {
        Self::InvalidCamera(err)
    }
}

impl TryFrom<SceneParser> for Scene {
    type Error = InvalidScene;

    fn try_from(sp: SceneParser) -> Result<Self, Self::Error> {
        let camera = Camera::try_from(sp.camera)?;
        let world = World::from(sp.world);

        Ok(Self { camera, world })
    }
}

impl std::error::Error for InvalidScene {}

impl std::fmt::Display for InvalidScene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParsingError(err) => write!(f, "{}", err.to_string()),
            Self::InvalidCamera(err) => write!(f, "{}", err.to_string()),
        }
    }
}

pub fn parse(input: &str) -> Result<Scene, InvalidScene> {
    let scene_parser: SceneParser = serde_json::from_str(input)?;
    let scene = Scene::try_from(scene_parser)?;

    Ok(scene)
}

impl Scene {
    pub fn render(&self) -> Canvas {
        self.camera.render(&self.world)
    }
}

#[cfg(test)]
mod tests {
    use engine::{
        matrix::Matrix,
        tuple::{Point, Vector},
    };

    use crate::tuple::{PointParser, VectorParser};

    use super::*;

    #[test]
    fn parsing_a_scene_with_a_camera_and_world() {
        let input = r#"
{
    "camera": {
        "width": 1920,
        "height": 1080,
        "field_of_view": 60,
        "position": {
            "x": 10,
            "y": 10,
            "z": 10
        }
    },
    "world": {
        "objects": [],
        "lights": []
    }
}
        "#;

        let output: SceneParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            SceneParser {
                camera: CameraParser {
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
                },
                world: WorldParser::default()
            }
        );
    }

    #[test]
    fn getting_a_world_from_a_parsed_world() {
        let input = r#"
{
    "camera": {
        "width": 1920,
        "height": 1080,
        "field_of_view": 60,
        "position": {
            "x": 10,
            "y": 10,
            "z": 10
        }
    },
    "world": {
        "objects": [],
        "lights": []
    }
}
        "#;

        let output: SceneParser = serde_json::from_str(input).unwrap();

        let mut expected_camera = Camera::new(1920, 1080, std::f64::consts::FRAC_PI_3);
        expected_camera.transform = Matrix::view(
            Point::new(10.0, 10.0, 10.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
        .unwrap();

        assert_eq!(
            Scene::try_from(output).unwrap(),
            Scene {
                camera: expected_camera,
                world: World {
                    lights: vec![],
                    objects: vec![]
                }
            }
        );
    }
}
