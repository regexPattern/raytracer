use serde::Deserialize;

use crate::material::{Material, Texture};

use super::{color::ColorParser, texture::TextureParser};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(default)]
struct MaterialParser {
    ambient: f64,
    diffuse: f64,
    reflective: f64,
    shininess: f64,
    specular: f64,
    texture: TextureParser,
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
    fn parsing_a_material() {}

    #[test]
    fn getting_a_material_from_a_parsed_material() {}

    #[test]
    fn the_default_material() {
        let input = r#"
{}
        "#;

        let output: MaterialParser = serde_json::from_str(input).unwrap();

        assert_eq!(Material::from(output), Material::default());
    }
}
