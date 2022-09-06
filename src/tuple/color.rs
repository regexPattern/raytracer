use std::ops::{Add, Mul, Sub};

use crate::utils;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub fn clamp(value: f64) -> u8 {
        match value {
            x if x <= 0.0 => 0,
            x if x >= 1.0 => 255,
            x => (x * 255.0) as u8,
        }
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        utils::approximately_eq(self.red, other.red)
            && utils::approximately_eq(self.green, other.green)
            && utils::approximately_eq(self.blue, other.blue)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.red - rhs.red,
            self.green - rhs.green,
            self.blue - rhs.blue,
        )
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_red_green_blue_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);

        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn clamp_color() {
        let c1 = 1.0;
        let c2 = 0.0;
        let c3 = 0.5;
        let c4 = -1.0;
        let c5 = 256.0;

        assert_eq!(Color::clamp(c1), 255);
        assert_eq!(Color::clamp(c2), 0);
        assert_eq!(Color::clamp(c3), 127);
        assert_eq!(Color::clamp(c4), 0);
        assert_eq!(Color::clamp(c5), 255);
    }

    #[test]
    fn getting_black_color() {
        assert_eq!(Color::black(), Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn getting_white_color() {
        assert_eq!(Color::white(), Color::new(1.0, 1.0, 1.0));
    }
}
