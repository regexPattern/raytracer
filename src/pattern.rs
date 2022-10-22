use crate::color::Color;
use crate::float;
use crate::matrix::Matrix;
use crate::shape::Shapes;
use crate::tuple::Point;

#[derive(Copy, Clone, Debug)]
pub struct Stripe {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix<4, 4>,
}

impl Stripe {
    pub fn stripe_at(&self, point: Point) -> Color {
        if float::approx(point.0.x.floor() % 2.0, 0.0) {
            return self.a;
        }

        self.b
    }

    pub fn stripe_at_object(&self, object: Shapes, world_point: Point) -> Color {
        let object_point = object.shape().transform.inverse() * world_point;
        let pattern_point = self.transform.inverse() * object_point;
        self.stripe_at(pattern_point)
    }
}

#[cfg(test)]
mod tests {
    use crate::color;
    use crate::material::Material;
    use crate::matrix;
    use crate::shape::Shape;

    use super::*;

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = Stripe {
            a: color::WHITE,
            b: color::BLACK,
            transform: matrix::IDENTITY4X4,
        };

        assert_eq!(pattern.a, color::WHITE);
        assert_eq!(pattern.b, color::BLACK);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Stripe {
            a: color::WHITE,
            b: color::BLACK,
            transform: matrix::IDENTITY4X4,
        };

        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 1.0, 0.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 2.0, 0.0)), color::WHITE);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Stripe {
            a: color::WHITE,
            b: color::BLACK,
            transform: matrix::IDENTITY4X4,
        };

        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 1.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 2.0)), color::WHITE);
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Stripe {
            a: color::WHITE,
            b: color::BLACK,
            transform: matrix::IDENTITY4X4,
        };

        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.9, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(1.0, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.stripe_at(Point::new(-0.1, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.stripe_at(Point::new(-1.0, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.stripe_at(Point::new(-1.1, 0.0, 0.0)), color::WHITE);
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let object = Shapes::Sphere(Shape {
            transform: Matrix::scaling(2.0, 2.0, 2.0),
            material: Material {
                pattern: Some(Stripe {
                    a: color::WHITE,
                    b: color::BLACK,
                    transform: matrix::IDENTITY4X4,
                }),
                ..Default::default()
            },
        });

        let pattern = object.shape().material.pattern.unwrap();
        let c = pattern.stripe_at_object(object, Point::new(1.5, 0.0, 0.0));

        assert_eq!(c, color::WHITE);
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Shapes::Sphere(Shape {
            material: Material {
                pattern: Some(Stripe {
                    a: color::WHITE,
                    b: color::BLACK,
                    transform: Matrix::scaling(2.0, 2.0, 2.0),
                }),
                ..Default::default()
            },
            ..Default::default()
        });

        let pattern = object.shape().material.pattern.unwrap();
        let c = pattern.stripe_at_object(object, Point::new(1.5, 0.0, 0.0));

        assert_eq!(c, color::WHITE);
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let object = Shapes::Sphere(Shape {
            transform: Matrix::scaling(2.0, 2.0, 2.0),
            material: Material {
                pattern: Some(Stripe {
                    a: color::WHITE,
                    b: color::BLACK,
                    transform: Matrix::translation(0.5, 0.0, 0.0),
                }),
                ..Default::default()
            },
        });

        let pattern = object.shape().material.pattern.unwrap();
        let c = pattern.stripe_at_object(object, Point::new(2.5, 0.0, 0.0));

        assert_eq!(c, color::WHITE);
    }
}
