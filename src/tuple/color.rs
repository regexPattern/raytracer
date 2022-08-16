use std::ops::{Add, Mul, Sub};

use crate::tuple::Tuple;

#[derive(Copy, Clone, Debug)]
pub struct Color(Tuple);

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color(Tuple::new(red, green, blue))
    }

    pub fn red(&self) -> u8 {
        Color::clamp(self.0.x)
    }

    pub fn green(&self) -> u8 {
        Color::clamp(self.0.y)
    }

    pub fn blue(&self) -> u8 {
        Color::clamp(self.0.z)
    }

    fn clamp(value: f64) -> u8 {
        match value {
            x if x < 0.0 => 0,
            x if x > 255.0 => 255,
            x => (x * 255.0) as u8,
        }
    }
}

impl From<Tuple> for Color {
    fn from(t: Tuple) -> Color {
        let Tuple { x, y, z } = t;
        let (red, green, blue) = (x, y, z);

        Color::new(red, green, blue)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.0 == other.0
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::from(self.0 + rhs.0)
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color::from(self.0 * rhs.0)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color::from(self.0 * rhs)
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        Color::from(self.0 - rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_color() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c.0.x, -0.5);
        assert_eq!(c.0.y, 0.4);
        assert_eq!(c.0.z, 1.7);
    }

    #[test]
    fn clamping_float() {
        let s1 = 0.0;
        let s2 = 0.2;
        let s3 = 1.0;

        assert_eq!(Color::clamp(s1), 0);
        assert_eq!(Color::clamp(s2), 51);
        assert_eq!(Color::clamp(s3), 255);

        let s4 = -1.0;
        let s5 = 256.0;
        let s6 = 0.5;

        assert_eq!(Color::clamp(s4), 0);
        assert_eq!(Color::clamp(s5), 255);
        assert_eq!(Color::clamp(s6), 127);
    }

    #[test]
    fn clamping_color() {
        let red = Color::new(1.0, 0.0, 0.0);
        let green = Color::new(0.0, 1.0, 0.0);
        let blue = Color::new(0.0, 0.0, 1.0);

        assert_eq!(red.red(), 255);
        assert_eq!(red.green(), 0);
        assert_eq!(red.blue(), 0);

        assert_eq!(green.green(), 255);
        assert_eq!(blue.blue(), 255);
    }

    #[test]
    fn adding_two_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn multiplying_color_by_float() {
        let c = Color::new(0.2, 0.3, 0.4);

        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8))
    }

    #[test]
    fn multiplying_two_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }

    #[test]
    fn subtracting_two_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }
}
