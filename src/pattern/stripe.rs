use crate::color::Color;
use crate::float;
use crate::tuple::Point;

use super::Design;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Stripe(pub Design);

impl Stripe {
    pub fn pattern_at(&self, pattern_point: Point) -> Color {
        if float::approx(pattern_point.0.x.floor() % 2.0, 0.0) {
            return self.0.a;
        }

        self.0.b
    }
}

#[cfg(test)]
mod tests {
    use crate::color;

    use super::*;

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = Stripe(Design::new(color::WHITE, color::BLACK));

        assert_eq!(pattern.0.a, color::WHITE);
        assert_eq!(pattern.0.b, color::BLACK);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Stripe(Design::new(color::WHITE, color::BLACK));

        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 1.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 2.0, 0.0)), color::WHITE);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Stripe(Design::new(color::WHITE, color::BLACK));

        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 1.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 2.0)), color::WHITE);
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Stripe(Design::new(color::WHITE, color::BLACK));

        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.9, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(1.0, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.pattern_at(Point::new(-0.1, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.pattern_at(Point::new(-1.0, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.pattern_at(Point::new(-1.1, 0.0, 0.0)), color::WHITE);
    }
}
