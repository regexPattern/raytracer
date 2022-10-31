use crate::color::Color;
use crate::float;
use crate::matrix::{self, Matrix};
use crate::tuple::Point;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Striped {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix<4, 4>,
}

impl Striped {
    pub fn new(a: Color, b: Color) -> Self {
        let transform = matrix::IDENTITY4X4;

        Self { a, b, transform }
    }

    pub fn stripe_at(&self, point: Point) -> Color {
        if float::approx(point.0.x.floor() % 2.0, 0.0) {
            return self.a;
        }

        self.b
    }
}

#[cfg(test)]
mod tests {
    use crate::color;
    use crate::tuple::Point;

    use super::*;

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = Striped::new(color::WHITE, color::BLACK);

        assert_eq!(pattern.a, color::WHITE);
        assert_eq!(pattern.b, color::BLACK);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Striped::new(color::WHITE, color::BLACK);

        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 1.0, 0.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 2.0, 0.0)), color::WHITE);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Striped::new(color::WHITE, color::BLACK);

        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 1.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 2.0)), color::WHITE);
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Striped::new(color::WHITE, color::BLACK);

        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.9, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(1.0, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.stripe_at(Point::new(-0.1, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.stripe_at(Point::new(-1.0, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.stripe_at(Point::new(-1.1, 0.0, 0.0)), color::WHITE);
    }
}
