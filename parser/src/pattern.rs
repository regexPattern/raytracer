use serde::Deserialize;

use core::{
    color::Color,
    matrix::Matrix,
    pattern::{Checker, Gradient, Pattern, Ring, Scheme, Stripe},
};

use super::{color::ColorParser, transform::MultipleTransformParser};

#[derive(Debug, Deserialize, PartialEq)]
pub struct SchemeParser {
    pub from: ColorParser,
    pub to: ColorParser,

    #[serde(default)]
    pub transform: MultipleTransformParser,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum PatternParser {
    Checker(SchemeParser),
    Gradient(SchemeParser),
    Ring(SchemeParser),
    Stripe(SchemeParser),
}

impl From<SchemeParser> for Scheme {
    fn from(s: SchemeParser) -> Self {
        let a = Color::from(s.from);
        let b = Color::from(s.to);
        let transform = Matrix::from(s.transform);

        Self { a, b, transform }
    }
}

impl From<PatternParser> for Pattern {
    fn from(p: PatternParser) -> Self {
        match p {
            PatternParser::Checker(sp) => Self::Checker(Checker(Scheme::from(sp))),
            PatternParser::Gradient(sp) => Self::Gradient(Gradient(Scheme::from(sp))),
            PatternParser::Ring(sp) => Self::Ring(Ring(Scheme::from(sp))),
            PatternParser::Stripe(sp) => Self::Stripe(Stripe(Scheme::from(sp))),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::color;

    use crate::transform::TransformParser;

    use super::*;

    #[test]
    fn parsing_a_scheme() {
        let input = r#"
{
    "from": {
        "red": 255,
        "green": 255,
        "blue": 255
    },
    "to": {
        "red": 0,
        "green": 0,
        "blue": 0
    },
    "transform": [
        {
            "type": "translation",
            "x": 1,
            "y": 2,
            "z": 3
        }
    ]
}
        "#;

        let output: SchemeParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            SchemeParser {
                from: ColorParser {
                    red: 255,
                    green: 255,
                    blue: 255,
                },
                to: ColorParser {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                transform: MultipleTransformParser(vec![TransformParser::Translation {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0
                }])
            }
        )
    }

    #[test]
    fn the_default_transformation_for_a_scheme() {
        let input = r#"
{
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

        let output: SchemeParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            SchemeParser {
                from: ColorParser {
                    red: 255,
                    green: 255,
                    blue: 255,
                },
                to: ColorParser {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                transform: MultipleTransformParser(vec![TransformParser::Identity]),
            }
        );
    }

    #[test]
    fn getting_a_scheme_from_a_parsed_scheme() {
        let input = r#"
{
    "from": {
        "red": 255,
        "green": 255,
        "blue": 255
    },
    "to": {
        "red": 0,
        "green": 0,
        "blue": 0
    },
    "transform": [
        {
            "type": "translation",
            "x": 1,
            "y": 2,
            "z": 3
        }
    ]
}
        "#;

        let output: SchemeParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            Scheme::from(output),
            Scheme {
                a: color::WHITE,
                b: color::BLACK,
                transform: Matrix::translation(1.0, 2.0, 3.0),
            }
        )
    }

    #[test]
    fn parsing_checker_pattern() {
        let input = r#"
{
    "type": "checker",
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

        let output: PatternParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            PatternParser::Checker(SchemeParser {
                from: ColorParser {
                    red: 255,
                    green: 255,
                    blue: 255,
                },
                to: ColorParser {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                transform: MultipleTransformParser(vec![TransformParser::Identity]),
            })
        )
    }

    #[test]
    fn parsing_gradient_pattern() {
        let input = r#"
{
    "type": "gradient",
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

        let output: PatternParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            PatternParser::Gradient(SchemeParser {
                from: ColorParser {
                    red: 255,
                    green: 255,
                    blue: 255,
                },
                to: ColorParser {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                transform: MultipleTransformParser(vec![TransformParser::Identity]),
            })
        )
    }

    #[test]
    fn parsing_ring_pattern() {
        let input = r#"
{
    "type": "ring",
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

        let output: PatternParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            PatternParser::Ring(SchemeParser {
                from: ColorParser {
                    red: 255,
                    green: 255,
                    blue: 255,
                },
                to: ColorParser {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                transform: MultipleTransformParser(vec![TransformParser::Identity]),
            })
        )
    }

    #[test]
    fn parsing_stripe_pattern() {
        let input = r#"
{
    "type": "stripe",
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

        let output: PatternParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            output,
            PatternParser::Stripe(SchemeParser {
                from: ColorParser {
                    red: 255,
                    green: 255,
                    blue: 255,
                },
                to: ColorParser {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                transform: MultipleTransformParser(vec![TransformParser::Identity])
            })
        )
    }

    #[test]
    fn getting_a_pattern_from_a_parsed_pattern() {
        let input = r#"
{
    "type": "checker",
    "from": {
        "red": 255,
        "green": 255,
        "blue": 255
    },
    "to": {
        "red": 0,
        "green": 0,
        "blue": 0
    },
    "transform": [
        {
            "type": "translation",
            "x": 1,
            "y": 2,
            "z": 3
        }
    ]
}
        "#;

        let output: PatternParser = serde_json::from_str(input).unwrap();

        assert_eq!(
            Pattern::from(output),
            Pattern::Checker(Checker(Scheme {
                a: color::WHITE,
                b: color::BLACK,
                transform: Matrix::translation(1.0, 2.0, 3.0),
            }))
        )
    }
}
