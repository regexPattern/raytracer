use crate::{
    color::Color,
    float,
    shape::Shape,
    transform::Transform,
    tuple::{Point, Tuple},
};

/// 3-dimensional pattern for materials.
///
/// 3-dimensional means that patterns are "cut out" by shapes instead of adapting each specific
/// pattern to the coordinate system adecuate to that shape. Pattern and texture mapping might be
/// added in the future.
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Pattern3D {
    /// A solid color.
    Solid(Color),

    /// A stripe pattern.
    Stripe(Pattern3DSpec),

    //// A gradient pattern.
    Gradient(Pattern3DSpec),

    /// A ring pattern.
    Ring(Pattern3DSpec),

    /// A checker pattern.
    Checker(Pattern3DSpec),
}

/// Specification describing a complex pattern's properties.
///
/// This includes patterns that use multiple colors and can be transformed relative to the shape
/// they are used in.
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pattern3DSpec {
    color_a: Color,
    color_b: Color,
    transform: Transform,
    transform_inverse: Transform,
}

impl Pattern3DSpec {
    /// Constructs a new pattern 3-dimensional spec.
    pub fn new(color_a: Color, color_b: Color, transform: Transform) -> Self {
        Self {
            color_a,
            color_b,
            transform,
            transform_inverse: transform.inverse(),
        }
    }
}

fn pattern_point(object: &Shape, transform_inverse: Transform, point: Point) -> Point {
    let object_point = object.as_ref().transform_inverse * point;
    transform_inverse * object_point
}

impl Pattern3D {
    pub(crate) fn color_at_object(&self, object: &Shape, point: Point) -> Color {
        self.color_at(pattern_point(object, self.transform_inverse(), point))
    }

    fn color_at(&self, point: Point) -> Color {
        let Point(Tuple { x, y, z, .. }) = point;

        match self {
            Self::Solid(c) => c.to_owned(),
            Self::Stripe(s) => {
                if float::approx(x.floor() % 2.0, 0.0) {
                    s.color_a
                } else {
                    s.color_b
                }
            }
            Self::Gradient(s) => s.color_a + (s.color_b - s.color_a) * (x - x.floor()),
            Self::Ring(s) => {
                if float::approx(x.hypot(z).floor() % 2.0, 0.0) {
                    s.color_a
                } else {
                    s.color_b
                }
            }
            Self::Checker(s) => {
                if float::approx((x.floor() + y.floor() + z.floor()) % 2.0, 0.0) {
                    s.color_a
                } else {
                    s.color_b
                }
            }
        }
    }

    fn transform_inverse(&self) -> Transform {
        match self {
            Self::Solid(_) => Default::default(),
            Self::Stripe(s) | Self::Gradient(s) | Self::Ring(s) | Self::Checker(s) => {
                s.transform_inverse
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color,
        shape::{ShapeBuilder, Sphere},
    };

    use super::*;

    #[derive(Debug)]
    struct MockPattern(Pattern3DSpec);

    impl Default for MockPattern {
        fn default() -> Self {
            Self(Pattern3DSpec::new(
                color::consts::WHITE,
                color::consts::BLACK,
                Default::default(),
            ))
        }
    }

    impl MockPattern {
        fn color_at_object(&self, object: &Shape, point: Point) -> Color {
            let pattern_point = pattern_point(object, self.0.transform.inverse(), point);

            Color {
                red: pattern_point.0.x,
                green: pattern_point.0.y,
                blue: pattern_point.0.z,
            }
        }
    }

    #[test]
    fn creating_a_stripe_pattern() {
        let p = Pattern3D::Stripe(Pattern3DSpec::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Default::default(),
        ));

        assert!(
            matches!(p, Pattern3D::Stripe(Pattern3DSpec { color_a, .. }) if color_a == color::consts::WHITE)
        );
        assert!(
            matches!(p, Pattern3D::Stripe(Pattern3DSpec { color_b, .. }) if color_b == color::consts::BLACK)
        );
        assert!(
            matches!(p, Pattern3D::Stripe(Pattern3DSpec { transform: t, .. }) if t == Default::default())
        );
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let p = Pattern3D::Stripe(Pattern3DSpec::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Default::default(),
        ));

        assert_eq!(p.color_at(Point::new(0.0, 0.0, 0.0)), color::consts::WHITE);
        assert_eq!(p.color_at(Point::new(0.0, 1.0, 0.0)), color::consts::WHITE);
        assert_eq!(p.color_at(Point::new(0.0, 2.0, 0.0)), color::consts::WHITE);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let p = Pattern3D::Stripe(Pattern3DSpec::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Default::default(),
        ));

        assert_eq!(p.color_at(Point::new(0.0, 0.0, 0.0)), color::consts::WHITE);
        assert_eq!(p.color_at(Point::new(0.0, 0.0, 1.0)), color::consts::WHITE);
        assert_eq!(p.color_at(Point::new(0.0, 0.0, 2.0)), color::consts::WHITE);
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let p = Pattern3D::Stripe(Pattern3DSpec::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Default::default(),
        ));

        assert_eq!(p.color_at(Point::new(0.0, 0.0, 0.0)), color::consts::WHITE);
        assert_eq!(p.color_at(Point::new(0.9, 0.0, 0.0)), color::consts::WHITE);
        assert_eq!(p.color_at(Point::new(1.0, 0.0, 0.0)), color::consts::BLACK);
        assert_eq!(p.color_at(Point::new(-0.1, 0.0, 0.0)), color::consts::BLACK);
        assert_eq!(p.color_at(Point::new(-1.0, 0.0, 0.0)), color::consts::BLACK);
        assert_eq!(p.color_at(Point::new(-1.1, 0.0, 0.0)), color::consts::WHITE);
    }

    #[test]
    fn stripes_with_object_transform() {
        let object = Shape::Sphere(Sphere::from(ShapeBuilder {
            transform: Transform::scaling(2.0, 2.0, 2.0).unwrap(),
            ..Default::default()
        }));

        let pattern = Pattern3D::Stripe(Pattern3DSpec::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Default::default(),
        ));

        let color_at = pattern.color_at_object(&object, Point::new(1.5, 0.0, 0.0));

        assert_eq!(color_at, color::consts::WHITE);
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Shape::Sphere(Default::default());

        let patter = Pattern3D::Stripe(Pattern3DSpec::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Transform::scaling(2.0, 2.0, 2.0).unwrap(),
        ));

        let color_at = patter.color_at_object(&object, Point::new(1.5, 0.0, 0.0));

        assert_eq!(color_at, color::consts::WHITE);
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let object = Shape::Sphere(Sphere::from(ShapeBuilder {
            transform: Transform::scaling(2.0, 2.0, 2.0).unwrap(),
            ..Default::default()
        }));

        let pattern = Pattern3D::Stripe(Pattern3DSpec::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Transform::translation(0.5, 0.0, 0.0),
        ));

        let color_at = pattern.color_at_object(&object, Point::new(2.5, 0.0, 0.0));

        assert_eq!(color_at, color::consts::WHITE);
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let object = Shape::Sphere(Sphere::from(ShapeBuilder {
            transform: Transform::scaling(2.0, 2.0, 2.0).unwrap(),
            ..Default::default()
        }));

        let pattern = MockPattern::default();

        let color_at = pattern.color_at_object(&object, Point::new(2.0, 3.0, 4.0));

        assert_eq!(
            color_at,
            Color {
                red: 1.0,
                green: 1.5,
                blue: 2.0
            }
        );
    }

    #[test]
    fn a_pattern_with_a_pattern_transformation() {
        let object = Shape::Sphere(Default::default());

        let pattern = MockPattern(Pattern3DSpec::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Transform::scaling(2.0, 2.0, 2.0).unwrap(),
        ));

        let color_at = pattern.color_at_object(&object, Point::new(2.0, 3.0, 4.0));

        assert_eq!(
            color_at,
            Color {
                red: 1.0,
                green: 1.5,
                blue: 2.0
            }
        );
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let object = Shape::Sphere(Sphere::from(ShapeBuilder {
            transform: Transform::scaling(2.0, 2.0, 2.0).unwrap(),
            ..Default::default()
        }));

        let pattern = MockPattern(Pattern3DSpec::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Transform::translation(0.5, 1.0, 1.5),
        ));

        let color_at = pattern.color_at_object(&object, Point::new(2.5, 3.0, 3.5));

        assert_eq!(
            color_at,
            Color {
                red: 0.75,
                green: 0.5,
                blue: 0.25
            }
        );
    }

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let pattern = Pattern3D::Gradient(Pattern3DSpec::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Default::default(),
        ));

        assert_eq!(
            pattern.color_at(Point::new(0.0, 0.0, 0.0)),
            color::consts::WHITE
        );

        assert_eq!(
            pattern.color_at(Point::new(0.25, 0.0, 0.0)),
            Color {
                red: 0.75,
                green: 0.75,
                blue: 0.75
            }
        );
        assert_eq!(
            pattern.color_at(Point::new(0.5, 0.0, 0.0)),
            Color {
                red: 0.5,
                green: 0.5,
                blue: 0.5
            }
        );
        assert_eq!(
            pattern.color_at(Point::new(0.75, 0.0, 0.0)),
            Color {
                red: 0.25,
                green: 0.25,
                blue: 0.25
            }
        );
    }

    #[test]
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = Pattern3D::Ring(Pattern3DSpec::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Default::default(),
        ));

        assert_eq!(
            pattern.color_at(Point::new(0.0, 0.0, 0.0)),
            color::consts::WHITE
        );

        assert_eq!(
            pattern.color_at(Point::new(1.0, 0.0, 0.0)),
            color::consts::BLACK
        );

        assert_eq!(
            pattern.color_at(Point::new(0.0, 0.0, 1.0)),
            color::consts::BLACK
        );

        assert_eq!(
            pattern.color_at(Point::new(0.708, 0.0, 0.708)),
            color::consts::BLACK
        );
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = Pattern3D::Checker(Pattern3DSpec::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Default::default(),
        ));

        assert_eq!(
            pattern.color_at(Point::new(0.0, 0.0, 0.0)),
            color::consts::WHITE
        );

        assert_eq!(
            pattern.color_at(Point::new(0.99, 0.0, 0.0)),
            color::consts::WHITE
        );

        assert_eq!(
            pattern.color_at(Point::new(1.01, 0.0, 0.0)),
            color::consts::BLACK
        );
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = Pattern3D::Checker(Pattern3DSpec::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Default::default(),
        ));

        assert_eq!(
            pattern.color_at(Point::new(0.0, 0.0, 0.0)),
            color::consts::WHITE
        );

        assert_eq!(
            pattern.color_at(Point::new(0.0, 0.99, 0.0)),
            color::consts::WHITE
        );

        assert_eq!(
            pattern.color_at(Point::new(0.0, 1.01, 0.0)),
            color::consts::BLACK
        );
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = Pattern3D::Checker(Pattern3DSpec::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Default::default(),
        ));

        assert_eq!(
            pattern.color_at(Point::new(0.0, 0.0, 0.0)),
            color::consts::WHITE
        );

        assert_eq!(
            pattern.color_at(Point::new(0.0, 0.0, 0.99)),
            color::consts::WHITE
        );

        assert_eq!(
            pattern.color_at(Point::new(0.0, 0.0, 1.01)),
            color::consts::BLACK
        );
    }
}
