use std::ops::{Add, Mul, Sub};

use serde::Deserialize;

use crate::float;

pub mod consts;

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(from = "ColorDeserializer")]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

#[derive(Debug, Deserialize)]
pub struct ColorDeserializer {
    red: u8,
    green: u8,
    blue: u8,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        float::approx(self.red, other.red)
            && float::approx(self.green, other.green)
            && float::approx(self.blue, other.blue)
    }
}

impl From<ColorDeserializer> for Color {
    fn from(value: ColorDeserializer) -> Self {
        let red = f64::from(value.red) / 255.0;
        let green = f64::from(value.green) / 255.0;
        let blue = f64::from(value.blue) / 255.0;

        Self { red, green, blue }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let red = self.red + rhs.red;
        let green = self.green + rhs.green;
        let blue = self.blue + rhs.blue;

        Self { red, green, blue }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let red = self.red - rhs.red;
        let green = self.green - rhs.green;
        let blue = self.blue - rhs.blue;

        Self { red, green, blue }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let red = self.red * rhs;
        let green = self.green * rhs;
        let blue = self.blue * rhs;

        Self { red, green, blue }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let red = self.red * rhs.red;
        let green = self.green * rhs.green;
        let blue = self.blue * rhs.blue;

        Self { red, green, blue }
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_de_tokens, Token};

    use super::*;

    use crate::assert_approx;

    #[test]
    fn colors_are_red_green_blue_tuples() {
        let c = Color {
            red: -0.5,
            green: 0.4,
            blue: 1.7,
        };

        assert_approx!(c.red, -0.5);
        assert_approx!(c.green, 0.4);
        assert_approx!(c.blue, 1.7);
    }

    #[test]
    fn adding_colors() {
        let c0 = Color {
            red: 0.9,
            green: 0.6,
            blue: 0.75,
        };

        let c1 = Color {
            red: 0.7,
            green: 0.1,
            blue: 0.25,
        };

        assert_eq!(
            c0 + c1,
            Color {
                red: 1.6,
                green: 0.7,
                blue: 1.0,
            }
        );
    }

    #[test]
    fn subtracting_colors() {
        let c0 = Color {
            red: 0.9,
            green: 0.6,
            blue: 0.75,
        };

        let c1 = Color {
            red: 0.7,
            green: 0.1,
            blue: 0.25,
        };

        assert_eq!(
            c0 - c1,
            Color {
                red: 0.2,
                green: 0.5,
                blue: 0.5,
            }
        );
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
        let c = Color {
            red: 0.2,
            green: 0.3,
            blue: 0.4,
        };

        assert_eq!(
            c * 2.0,
            Color {
                red: 0.4,
                green: 0.6,
                blue: 0.8,
            }
        );
        assert_eq!(c * 2.0, 2.0 * c);
    }

    #[test]
    fn multiplying_two_colors() {
        let c0 = Color {
            red: 1.0,
            green: 0.2,
            blue: 0.4,
        };

        let c1 = Color {
            red: 0.9,
            green: 1.0,
            blue: 0.1,
        };

        assert_eq!(
            c0 * c1,
            Color {
                red: 0.9,
                green: 0.2,
                blue: 0.04,
            }
        );
        assert_eq!(c0 * c1, c1 * c0);
    }

    #[test]
    fn deserializing_a_color() {
        assert_de_tokens(
            &Color {
                red: 0.0,
                green: 0.49803,
                blue: 1.0,
            },
            &[
                Token::Struct {
                    name: "ColorDeserializer",
                    len: 3,
                },
                Token::Str("red"),
                Token::U8(0),
                Token::Str("green"),
                Token::U8(127),
                Token::Str("blue"),
                Token::U8(255),
                Token::StructEnd,
            ],
        );
    }
}
