use crate::{
    color::Color,
    matrix::{self, Matrix},
    shape::Shapes,
    tuple::Point,
};

mod checker;
mod gradient;
mod ring;
mod stripe;

pub use checker::Checker;
pub use gradient::Gradient;
pub use ring::Ring;
pub use stripe::Stripe;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Patterns {
    Checker(Checker),
    Gradient(Gradient),
    Ring(Ring),
    Stripe(Stripe),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Scheme {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix<4, 4>,
}

impl Patterns {
    pub fn pattern_at(&self, object: &Shapes, world_point: Point) -> Color {
        let pattern_point = self.pattern_point(object, world_point);

        match self {
            Self::Checker(c) => c.pattern_at(pattern_point),
            Self::Gradient(g) => g.pattern_at(pattern_point),
            Self::Ring(r) => r.pattern_at(pattern_point),
            Self::Stripe(s) => s.pattern_at(pattern_point),
        }
    }

    fn pattern_point(&self, object: &Shapes, world_point: Point) -> Point {
        let object_point = object.shape().transform.inverse() * world_point;

        self.transform().inverse() * object_point
    }

    const fn transform(&self) -> Matrix<4, 4> {
        match self {
            Self::Checker(c) => c.0.transform,
            Self::Gradient(g) => g.0.transform,
            Self::Ring(r) => r.0.transform,
            Self::Stripe(s) => s.0.transform,
        }
    }
}

impl Scheme {
    pub const fn new(a: Color, b: Color) -> Self {
        let transform = matrix::IDENTITY4X4;

        Self { a, b, transform }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color,
        shape::{Figure, Sphere},
    };

    use super::*;

    fn test_pattern(transform: Matrix<4, 4>) -> Patterns {
        Patterns::Stripe(Stripe(Scheme {
            a: color::WHITE,
            b: color::BLACK,
            transform,
        }))
    }

    fn test_pattern_pattern_at(pattern: Patterns, object: &Shapes, world_point: Point) -> Color {
        let pattern_point = pattern.pattern_point(object, world_point);
        Color {
            red: pattern_point.0.x,
            green: pattern_point.0.y,
            blue: pattern_point.0.z,
        }
    }

    #[test]
    fn the_default_pattern_transformation() {
        let pattern = Scheme::new(color::WHITE, color::BLACK);

        assert_eq!(pattern.transform, matrix::IDENTITY4X4);
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let shape = Shapes::Sphere(Sphere(Figure {
            transform: Matrix::scaling(2.0, 2.0, 2.0),
            ..Default::default()
        }));

        let pattern = test_pattern(matrix::IDENTITY4X4);

        let color = test_pattern_pattern_at(pattern, &shape, Point::new(2.0, 3.0, 4.0));

        assert_eq!(
            color,
            Color {
                red: 1.0,
                green: 1.5,
                blue: 2.0,
            }
        );
    }

    #[test]
    fn a_pattern_with_a_pattern_transformation() {
        let shape = Shapes::Sphere(Sphere::default());

        let pattern = test_pattern(Matrix::scaling(2.0, 2.0, 2.0));

        let color = test_pattern_pattern_at(pattern, &shape, Point::new(2.0, 3.0, 4.0));

        assert_eq!(
            color,
            Color {
                red: 1.0,
                green: 1.5,
                blue: 2.0,
            }
        );
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let shape = Shapes::Sphere(Sphere(Figure {
            transform: Matrix::scaling(2.0, 2.0, 2.0),
            ..Default::default()
        }));

        let pattern = test_pattern(Matrix::translation(0.5, 1.0, 1.5));

        let color = test_pattern_pattern_at(pattern, &shape, Point::new(2.5, 3.0, 3.5));

        assert_eq!(
            color,
            Color {
                red: 0.75,
                green: 0.5,
                blue: 0.25,
            }
        );
    }
}
