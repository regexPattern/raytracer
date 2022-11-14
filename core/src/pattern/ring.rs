use crate::{
    color::Color,
    float,
    tuple::Point,
};

use super::Scheme;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ring(pub Scheme);

impl Ring {
    pub fn pattern_at(&self, pattern_point: Point) -> Color {
        let distance = pattern_point.0.x.hypot(pattern_point.0.z);

        if float::approx(distance.floor() % 2.0, 0.0) {
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
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = Ring(Scheme::new(color::WHITE, color::BLACK));

        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(1.0, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 1.0)), color::BLACK);
        assert_eq!(
            pattern.pattern_at(Point::new(0.708, 0.0, 0.708)),
            color::BLACK
        );
    }
}
