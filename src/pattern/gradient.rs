use crate::{
    color::Color,
    tuple::Point,
};

use super::Scheme;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Gradient(pub Scheme);

impl Gradient {
    pub fn pattern_at(&self, pattern_point: Point) -> Color {
        let distance = self.0.b - self.0.a;
        let fraction = pattern_point.0.x - pattern_point.0.x.floor();

        self.0.a + distance * fraction
    }
}

#[cfg(test)]
mod tests {
    use crate::color;

    use super::*;

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let pattern = Gradient(Scheme::new(color::WHITE, color::BLACK));

        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(
            pattern.pattern_at(Point::new(0.25, 0.0, 0.0)),
            Color {
                red: 0.75,
                green: 0.75,
                blue: 0.75,
            }
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.5, 0.0, 0.0)),
            Color {
                red: 0.5,
                green: 0.5,
                blue: 0.5,
            }
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.75, 0.0, 0.0)),
            Color {
                red: 0.25,
                green: 0.25,
                blue: 0.25,
            }
        );
    }
}
