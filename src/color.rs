use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color(f64, f64, f64);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(r, g, b)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

#[cfg(test)]
mod create {
    use super::*;

    #[test]
    fn create_color() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c.0, -0.5);
        assert_eq!(c.1, 0.4);
        assert_eq!(c.2, 1.7);
    }
}

#[cfg(test)]
mod ops {
    use super::*;

    #[test]
    fn adding_two_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_two_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_color_by_scalar() {}

    #[test]
    fn multiplying_two_colors() {}
}
