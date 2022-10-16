use std::ops::{Add, Mul, Sub};

use crate::float;

pub const WHITE: Color = Color {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};

pub const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

pub const RED: Color = Color {
    red: 1.0,
    green: 0.0,
    blue: 0.0,
};

pub const GREEN: Color = Color {
    red: 0.0,
    green: 1.0,
    blue: 0.0,
};

pub const BLUE: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 1.0,
};

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RGBColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        float::approx(self.red, other.red)
            && float::approx(self.green, other.green)
            && float::approx(self.blue, other.blue)
    }
}

impl From<RGBColor> for Color {
    fn from(rgb: RGBColor) -> Self {
        let red = f64::from(rgb.red) / 255.0;
        let green = f64::from(rgb.green) / 255.0;
        let blue = f64::from(rgb.blue) / 255.0;

        Self { red, green, blue }
    }
}

impl From<Color> for RGBColor {
    fn from(color: Color) -> Self {
        let red = (color.red * 255.0) as u8;
        let green = (color.green * 255.0) as u8;
        let blue = (color.blue * 255.0) as u8;

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
    use crate::assert_approx;

    use super::*;

    #[test]
    fn colores_are_red_green_blue_tuples() {
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
                blue: 1.0
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
                blue: 0.5
            }
        );
    }

    #[test]
    fn multiplying_color_by_scalar() {
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
                blue: 0.8
            }
        );
        assert_eq!(
            c * 2.0,
            2.0 * c,
            "`Color` and `f64` multiplication is commutative"
        )
    }

    #[test]
    fn multiplying_colors() {
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
                blue: 0.04
            }
        );
    }

    #[test]
    fn constructing_colors_from_rgb_values() {
        let c1 = RGBColor {
            red: 255,
            green: 127,
            blue: 0,
        };
        let c2 = RGBColor {
            red: 241,
            green: 113,
            blue: 14,
        };

        assert_eq!(
            Color::from(c1),
            Color {
                red: 1.0,
                green: 0.49803,
                blue: 0.0
            }
        );
        assert_eq!(
            Color::from(c2),
            Color {
                red: 0.94509,
                green: 0.44313,
                blue: 0.0549,
            }
        );
    }

    #[test]
    fn getting_a_colors_rgb_values() {
        let c1 = Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        };
        let c2 = Color {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        };
        let c3 = Color {
            red: 0.5,
            green: 0.75,
            blue: 0.25,
        };

        assert_eq!(
            RGBColor::from(c1),
            RGBColor {
                red: 0,
                green: 0,
                blue: 0
            }
        );
        assert_eq!(
            RGBColor::from(c2),
            RGBColor {
                red: 255,
                green: 255,
                blue: 255
            }
        );
        assert_eq!(
            RGBColor::from(c3),
            RGBColor {
                red: 127,
                green: 191,
                blue: 63
            }
        );
    }
}
