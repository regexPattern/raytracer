use serde::Deserialize;

use raytracer::{
    material::Material,
    matrix::Matrix,
    shape::{Figure, Plane, Shape, Sphere},
};

use crate::{material::MaterialParser, transform::MultipleTransformParser};

#[derive(Debug, Deserialize, Default, PartialEq)]
#[serde(default)]
pub struct FigureParser {
    material: MaterialParser,
    transforms: MultipleTransformParser,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ShapeParser {
    Plane(FigureParser),
    Sphere(FigureParser),
}

impl From<FigureParser> for Figure {
    fn from(f: FigureParser) -> Self {
        let material = Material::from(f.material);
        let transform = Matrix::from(f.transforms);

        Self {
            material,
            transform,
        }
    }
}

impl From<ShapeParser> for Shape {
    fn from(s: ShapeParser) -> Self {
        match s {
            ShapeParser::Plane(fp) => Self::Plane(Plane(Figure::from(fp))),
            ShapeParser::Sphere(fp) => Self::Sphere(Sphere(Figure::from(fp))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::transform::TransformParser;

    use super::*;

    #[test]
    fn the_default_figure() {
        let input = r#"
{}
        "#;

        let output: FigureParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            FigureParser {
                material: MaterialParser::default(),
                transforms: MultipleTransformParser::default(),
            }
        );
    }

    #[test]
    fn parsing_a_figure() {
        let input = r#"
{
    "material": {
        "ambient": 1,
        "diffuse": 2,
        "reflective": 3
    },
    "transforms": [
        {
            "type": "rotation_x",
            "degrees": 1.25
        }
    ]
}
        "#;

        let output: FigureParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            FigureParser {
                material: MaterialParser {
                    ambient: 1.0,
                    diffuse: 2.0,
                    reflective: 3.0,
                    ..Default::default()
                },
                transforms: MultipleTransformParser(vec![TransformParser::RotationX {
                    degrees: 1.25
                }])
            }
        );
    }

    #[test]
    fn getting_a_figure_from_a_parsed_figure() {
        let input = r#"
{
    "material": {
        "ambient": 1,
        "diffuse": 2,
        "reflective": 3
    },
    "transforms": [
        {
            "type": "rotation_x",
            "degrees": 1.25
        }
    ]
}
        "#;

        let output: FigureParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            Figure::from(output),
            Figure {
                material: Material {
                    ambient: 1.0,
                    diffuse: 2.0,
                    reflective: 3.0,
                    ..Default::default()
                },
                transform: Matrix::rotation_x(1.25_f64.to_radians())
            }
        )
    }

    #[test]
    fn parsing_a_plane() {
        let input = r#"
{
    "type": "plane"
}
        "#;

        let output: ShapeParser = serde_json::from_str(input).unwrap();

        assert_eq!(output, ShapeParser::Plane(FigureParser::default()));
    }

    #[test]
    fn parsing_a_sphere() {
        let input = r#"
{
    "type": "sphere"
}
        "#;

        let output: ShapeParser = serde_json::from_str(input).unwrap();

        assert_eq!(output, ShapeParser::Sphere(FigureParser::default()));
    }

    #[test]
    fn getting_a_shape_from_a_parsed_shape() {
        let input = r#"
{
    "type": "sphere",
    "material": {
        "ambient": 1,
        "diffuse": 2,
        "reflective": 3
    }
}
        "#;

        let output: ShapeParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            Shape::from(output),
            Shape::Sphere(Sphere(Figure {
                material: Material {
                    ambient: 1.0,
                    diffuse: 2.0,
                    reflective: 3.0,
                    ..Default::default()
                },
                ..Default::default()
            }))
        )
    }
}
