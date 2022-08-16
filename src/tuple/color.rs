use std::ops::{Add, Mul, Sub};

use crate::tuple::{Scalar, Tuple};

#[derive(Debug)]
struct Color(Tuple);

impl Color {
    fn new(red: f64, green: f64, blue: f64) -> Color {
        Color(Tuple::new(red, green, blue))
    }

    fn red(&self) -> Scalar {
        self.0.x
    }

    fn green(&self) -> Scalar {
        self.0.y
    }

    fn blue(&self) -> Scalar {
        self.0.z
    }
}

impl From<Tuple> for Color {
    fn from(tuple: Tuple) -> Color {
        let (red, green, blue) = tuple.coordinates();

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

    fn add(self, rhs: Color) -> Color {
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

impl Mul<Scalar> for Color {
    type Output = Color;

    fn mul(self, rhs: Scalar) -> Color {
        self * rhs.0
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

        assert_eq!(c.red(), Scalar(-0.5));
        assert_eq!(c.green(), Scalar(0.4));
        assert_eq!(c.blue(), Scalar(1.7));
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
    fn multiplying_color_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);

        assert_eq!(c * Scalar(2.0), Color::new(0.4, 0.6, 0.8))
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
