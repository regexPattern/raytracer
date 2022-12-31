pub mod consts;

use std::ops::{Add, Mul, Sub};

use crate::utils;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        utils::approx(self.red, other.red)
            && utils::approx(self.green, other.green)
            && utils::approx(self.blue, other.blue)
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
        let c1 = Color {
            red: 0.9,
            green: 0.6,
            blue: 0.75,
        };

        let c2 = Color {
            red: 0.7,
            green: 0.1,
            blue: 0.25,
        };

        assert_eq!(
            c1 + c2,
            Color {
                red: 1.6,
                green: 0.7,
                blue: 1.0,
            }
        );
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color {
            red: 0.9,
            green: 0.6,
            blue: 0.75,
        };

        let c2 = Color {
            red: 0.7,
            green: 0.1,
            blue: 0.25,
        };

        assert_eq!(
            c1 - c2,
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
        let c1 = Color {
            red: 1.0,
            green: 0.2,
            blue: 0.4,
        };

        let c2 = Color {
            red: 0.9,
            green: 1.0,
            blue: 0.1,
        };

        assert_eq!(
            c1 * c2,
            Color {
                red: 0.9,
                green: 0.2,
                blue: 0.04,
            }
        );
        assert_eq!(c1 * c2, c2 * c1);
    }
}
