use crate::color::Color;
use crate::float;
use crate::matrix::{self, Matrix};
use crate::shape::Shape;
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

    pub fn stripe_at_object(&self, object: Shape, world_point: Point) -> Color {
        let object_point = object.shape().transform.inverse() * world_point;
        let pattern_point = self.transform.inverse() * object_point;
        self.stripe_at(pattern_point)
    }
}

#[cfg(test)]
mod tests {
    use crate::color;
    use crate::material::{Material, Texture};
    use crate::shape::ShapeProps;

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

    #[test]
    fn stripes_with_an_object_transformation() {
        let object = Shape::Sphere(ShapeProps {
            transform: Matrix::scaling(2.0, 2.0, 2.0),
            material: Material {
                texture: Texture::Pattern(Striped::new(color::WHITE, color::BLACK)),
                ..Default::default()
            },
        });

        // TODO: More elegant way to manage this?
        if let Texture::Pattern(pattern) = object.shape().material.texture {
            let c = pattern.stripe_at_object(object, Point::new(1.5, 0.0, 0.0));
            assert_eq!(c, color::WHITE);
        } else {
            panic!();
        }
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Shape::Sphere(ShapeProps {
            material: Material {
                texture: Texture::Pattern(Striped {
                    a: color::WHITE,
                    b: color::BLACK,
                    transform: Matrix::scaling(2.0, 2.0, 2.0),
                }),
                ..Default::default()
            },
            ..Default::default()
        });

        if let Texture::Pattern(pattern) = object.shape().material.texture {
            let c = pattern.stripe_at_object(object, Point::new(1.5, 0.0, 0.0));
            assert_eq!(c, color::WHITE);
        } else {
            panic!()
        }
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let object = Shape::Sphere(ShapeProps {
            transform: Matrix::scaling(2.0, 2.0, 2.0),
            material: Material {
                texture: Texture::Pattern(Striped {
                    a: color::WHITE,
                    b: color::BLACK,
                    transform: Matrix::translation(0.5, 0.0, 0.0),
                }),
                ..Default::default()
            },
        });

        if let Texture::Pattern(pattern) = object.shape().material.texture {
            let c = pattern.stripe_at_object(object, Point::new(2.5, 0.0, 0.0));
            assert_eq!(c, color::WHITE);
        } else {
            panic!()
        }
    }
}
