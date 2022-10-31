mod striped;

use crate::color::Color;
use crate::float;
use crate::matrix::{self, Matrix};
use crate::shape::Shape;
use crate::tuple::Point;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Pattern {
    Striped(Striped),
}

impl Pattern {
    fn pattern_point(&self, object: Shape, world_point: Point) -> Point {
        let object_point = object.shape().transform.inverse() * world_point;
        self.transform().inverse() * object_point
    }

    fn transform(&self) -> Matrix<4, 4> {
        match self {
            Pattern::Striped(s) => s.transform,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Design {
    pub transform: Matrix<4, 4>,
}

impl Default for Design {
    fn default() -> Self {
        Self {
            transform: matrix::IDENTITY4X4,
        }
    }
}

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

    pub fn stripe_at_object(&self, object: Shape, world_point: Point) -> Color {
        let object_point = object.shape().transform.inverse() * world_point;
        let pattern_point = self.transform.inverse() * object_point;
        self.stripe_at(pattern_point)
    }
}

#[cfg(test)]
mod tests {
    use crate::color;
    use crate::shape::{Figure, Sphere};

    use super::*;

    fn test_pattern(transform: Matrix<4, 4>) -> Pattern {
        Pattern::Striped(Striped {
            a: color::WHITE,
            b: color::BLACK,
            transform,
        })
    }

    fn test_pattern_pattern_at(pattern: Pattern, object: Shape, world_point: Point) -> Color {
        let pattern_point = pattern.pattern_point(object, world_point);
        Color {
            red: pattern_point.0.x,
            green: pattern_point.0.y,
            blue: pattern_point.0.z,
        }
    }

    #[test]
    fn the_default_pattern_transformation() {
        let pattern = Design::default();

        assert_eq!(pattern.transform, matrix::IDENTITY4X4);
    }

    #[test]
    fn assigning_a_transformation() {
        let mut pattern = Design::default();
        let transform = Matrix::translation(1.0, 2.0, 3.0);

        pattern.transform = transform;

        assert_eq!(pattern.transform, transform);
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let shape = Shape::Sphere(Sphere(Figure {
            transform: Matrix::scaling(2.0, 2.0, 2.0),
            ..Default::default()
        }));

        let pattern = test_pattern(matrix::IDENTITY4X4);

        let color = test_pattern_pattern_at(pattern, shape, Point::new(2.0, 3.0, 4.0));

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
        let shape = Shape::Sphere(Sphere::default());

        let pattern = test_pattern(Matrix::scaling(2.0, 2.0, 2.0));

        let color = test_pattern_pattern_at(pattern, shape, Point::new(2.0, 3.0, 4.0));

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
        let shape = Shape::Sphere(Sphere(Figure {
            transform: Matrix::scaling(2.0, 2.0, 2.0),
            ..Default::default()
        }));

        let pattern = test_pattern(Matrix::translation(0.5, 1.0, 1.5));

        let color = test_pattern_pattern_at(pattern, shape, Point::new(2.5, 3.0, 3.5));

        assert_eq!(
            color,
            Color {
                red: 0.75,
                green: 0.5,
                blue: 0.25,
            }
        );
    }

    #[test] // MOVE TO OWN FILE
    fn creating_a_stripe_pattern() {
        let pattern = Striped::new(color::WHITE, color::BLACK);

        assert_eq!(pattern.a, color::WHITE);
        assert_eq!(pattern.b, color::BLACK);
    }

    #[test] // MOVE TO OWN FILE
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Striped::new(color::WHITE, color::BLACK);

        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 1.0, 0.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 2.0, 0.0)), color::WHITE);
    }

    #[test] // MOVE TO OWN FILE
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Striped::new(color::WHITE, color::BLACK);

        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 1.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 2.0)), color::WHITE);
    }

    #[test] // MOVE TO OWN FILE
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
