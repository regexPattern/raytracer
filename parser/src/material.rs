use serde::Deserialize;

use core::material::{Material, Texture};

use super::{color::ColorParser, texture::TextureParser};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(default)]
pub struct MaterialParser {
    pub ambient: f64,
    pub diffuse: f64,
    pub reflective: f64,
    pub shininess: f64,
    pub specular: f64,
    pub texture: TextureParser,
}

impl Default for MaterialParser {
    fn default() -> Self {
        let Material {
            ambient,
            diffuse,
            reflective,
            shininess,
            specular,
            ..
        } = Material::default();

        let texture = TextureParser::Color(ColorParser {
            red: 255,
            green: 255,
            blue: 255,
        });

        Self {
            ambient,
            diffuse,
            reflective,
            shininess,
            specular,
            texture,
        }
    }
}

impl From<MaterialParser> for Material {
    fn from(m: MaterialParser) -> Self {
        let MaterialParser {
            ambient,
            diffuse,
            reflective,
            shininess,
            specular,
            texture,
        } = m;

        let texture = Texture::from(texture);

        Self {
            ambient,
            diffuse,
            reflective,
            shininess,
            specular,
            texture,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_a_material() {
        let input = r#"
{
    "ambient": 1,
    "diffuse": 2,
    "reflective": 3,
    "shininess": 4,
    "specular": 5,
    "texture": {
        "red": 255,
        "green": 255,
        "blue": 255
    }
}
        "#;

        let output: MaterialParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            MaterialParser {
                ambient: 1.0,
                diffuse: 2.0,
                reflective: 3.0,
                shininess: 4.0,
                specular: 5.0,
                texture: TextureParser::Color(ColorParser {
                    red: 255,
                    green: 255,
                    blue: 255
                })
            }
        );
    }

    #[test]
    fn getting_a_material_from_a_parsed_material() {
        let input = r#"
{
    "ambient": 1,
    "diffuse": 2,
    "reflective": 3
}
        "#;

        let output: MaterialParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            Material::from(output),
            Material {
                ambient: 1.0,
                diffuse: 2.0,
                reflective: 3.0,
                ..Material::default()
            }
        );
    }

    #[test]
    fn the_default_material() {
        let input = r#"
{}
        "#;

        let output: MaterialParser = serde_json::from_str(input).unwrap();

        assert_eq!(Material::from(output), Material::default());
    }
}
