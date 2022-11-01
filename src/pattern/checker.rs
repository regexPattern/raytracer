use crate::color::Color;
use crate::float;
use crate::tuple::Point;

use super::Scheme;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Checker(pub Scheme);

impl Checker {
    pub fn pattern_at(&self, pattern_point: Point) -> Color {
        let sum = pattern_point.0.x.floor() + pattern_point.0.y.floor() + pattern_point.0.z.floor();

        if float::approx(sum % 2.0, 0.0) {
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
    fn checkers_should_repeat_in_x() {
        let pattern = Checker(Scheme::new(color::WHITE, color::BLACK));

        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.99, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(1.01, 0.0, 0.0)), color::BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = Checker(Scheme::new(color::WHITE, color::BLACK));

        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.99, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 1.01, 0.0)), color::BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = Checker(Scheme::new(color::WHITE, color::BLACK));

        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.99)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 1.01)), color::BLACK);
    }
}
