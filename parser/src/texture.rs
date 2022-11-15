use serde::Deserialize;

use engine::{color::Color, material::Texture, pattern::Pattern};

use crate::{color::ColorParser, pattern::PatternParser};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum TextureParser {
    Color(ColorParser),
    Pattern(PatternParser),
}

impl From<TextureParser> for Texture {
    fn from(t: TextureParser) -> Self {
        match t {
            TextureParser::Color(cp) => Self::Color(Color::from(cp)),
            TextureParser::Pattern(pp) => Self::Pattern(Pattern::from(pp)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        pattern::SchemeParser,
        transform::{MultipleTransformParser, TransformParser},
    };

    use super::*;

    #[test]
    fn parsing_a_color_texture() {
        let input = r#"
{
    "type": "color",
    "red": 255,
    "green": 127,
    "blue": 0
}
        "#;

        let output: TextureParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            TextureParser::Color(ColorParser {
                red: 255,
                green: 127,
                blue: 0,
            })
        );
    }

    #[test]
    fn parsing_a_pattern_texture() {
        let input = r#"
{
    "type": "pattern",
    "pattern": "gradient",
    "from": {
        "red": 255,
        "green": 255,
        "blue": 255
    },
    "to": {
        "red": 0,
        "green": 0,
        "blue": 0
    }
}
        "#;

        let output: TextureParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            TextureParser::Pattern(PatternParser::Gradient(SchemeParser {
                from: ColorParser {
                    red: 255,
                    green: 255,
                    blue: 255
                },
                to: ColorParser {
                    red: 0,
                    green: 0,
                    blue: 0
                },
                transform: MultipleTransformParser(vec![TransformParser::Identity]),
            }))
        );
    }

    #[test]
    fn getting_a_texture_from_a_parsed_texture() {
        let input = r#"
{
    "type": "color",
    "red": 255,
    "green": 0,
    "blue": 0
}
        "#;

        let output: TextureParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            Texture::from(output),
            Texture::Color(Color {
                red: 1.0,
                green: 0.0,
                blue: 0.0,
            })
        );
    }
}
