use serde::Deserialize;

use core::color::Color;

#[derive(Debug, Deserialize, PartialEq)]
pub struct ColorParser {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl From<ColorParser> for Color {
    fn from(c: ColorParser) -> Self {
        let red = f64::from(c.red) / 255.0;
        let green = f64::from(c.green) / 255.0;
        let blue = f64::from(c.blue) / 255.0;

        Self { red, green, blue }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_a_color() {
        let input = r#"
{
    "red": 255,
    "green": 127,
    "blue": 0
}
        "#;

        let output: ColorParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            ColorParser {
                red: 255,
                green: 127,
                blue: 0,
            }
        );
    }

    #[test]
    fn getting_a_color_from_a_parsed_color() {
        let input = r#"
{
    "red": 255,
    "green": 127,
    "blue": 0
}
        "#;

        let output: ColorParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            Color::from(output),
            Color {
                red: 1.0,
                green: 0.49803,
                blue: 0.0,
            }
        );
    }

    #[test]
    fn parsing_a_color_with_negative_values() {
        let input = r#"
{
    "red": -255,
    "green": -127,
    "blue": 0
}
        "#;

        let output: serde_json::Result<ColorParser> = serde_json::from_str(input);

        let expected = match output {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        };

        assert_eq!(
            expected,
            Err("invalid value: integer `-255`, expected u8 at line 3 column 15".to_string())
        );
    }
}
