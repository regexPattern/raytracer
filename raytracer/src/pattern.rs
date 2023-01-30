use crate::{
    color::Color,
    float,
    shape::Shape,
    transform::Transform,
    tuple::{Point, Tuple},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Schema {
    pub from: Color,
    pub to: Color,
    pub transform: Transform,
    pub transform_inverse: Transform,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern3D {
    Solid(Color),
    Stripe(Schema),
    Gradient(Schema),
    Ring(Schema),
    Checker(Schema),
}

impl Schema {
    pub fn new(from: Color, to: Color, transform: Transform) -> Self {
        Self {
            from,
            to,
            transform,
            transform_inverse: transform.inverse(),
        }
    }
}

impl Pattern3D {
    pub(crate) fn color_at_object(&self, object: &Shape, point: Point) -> Color {
        self.color_at(pattern_point(object, self.transform().inverse(), point))
    }

    fn color_at(&self, point: Point) -> Color {
        let Point(Tuple { x, y, z, .. }) = point;

        match self {
            Self::Solid(c) => c.to_owned(),
            Self::Stripe(s) => {
                if float::approx(x.floor() % 2.0, 0.0) {
                    s.from
                } else {
                    s.to
                }
            }
            Self::Gradient(s) => s.from + (s.to - s.from) * (x - x.floor()),
            Self::Ring(s) => {
                if float::approx(x.hypot(z).floor() % 2.0, 0.0) {
                    s.from
                } else {
                    s.to
                }
            }
            Self::Checker(s) => {
                if float::approx((x.floor() + y.floor() + z.floor()) % 2.0, 0.0) {
                    s.from
                } else {
                    s.to
                }
            }
        }
    }

    fn transform(&self) -> Transform {
        match self {
            Self::Solid(_) => Transform::default(),
            Self::Stripe(s) | Self::Gradient(s) | Self::Ring(s) | Self::Checker(s) => s.transform,
        }
    }
}

fn pattern_point(object: &Shape, transform_inverse: Transform, point: Point) -> Point {
    let object_point = object.as_ref().transform_inverse * point;
    transform_inverse * object_point
}

#[cfg(test)]
mod tests {
    use crate::{
        color,
        shape::sphere::{Sphere, SphereBuilder},
    };

    use super::*;

    #[derive(Debug)]
    struct TestPattern(Schema);

    impl Default for TestPattern {
        fn default() -> Self {
            Self(Schema::new(
                color::consts::WHITE,
                color::consts::BLACK,
                Default::default(),
            ))
        }
    }

    impl TestPattern {
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
        let p = Pattern3D::Stripe(Schema::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Default::default(),
        ));

        assert!(
            matches!(p, Pattern3D::Stripe(Schema { from, .. }) if from == color::consts::WHITE)
        );
        assert!(matches!(p, Pattern3D::Stripe(Schema { to, .. }) if to == color::consts::BLACK));
        assert!(
            matches!(p, Pattern3D::Stripe(Schema { transform: t, .. }) if t == Default::default())
        );
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let p = Pattern3D::Stripe(Schema::new(
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
        let p = Pattern3D::Stripe(Schema::new(
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
        let p = Pattern3D::Stripe(Schema::new(
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
        let o = Shape::Sphere(Sphere::from(SphereBuilder {
            transform: Transform::scaling(2.0, 2.0, 2.0).unwrap(),
            ..Default::default()
        }));

        let p = Pattern3D::Stripe(Schema::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Default::default(),
        ));

        let c = p.color_at_object(&o, Point::new(1.5, 0.0, 0.0));

        assert_eq!(c, color::consts::WHITE);
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let o = Shape::Sphere(Default::default());

        let p = Pattern3D::Stripe(Schema::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Transform::scaling(2.0, 2.0, 2.0).unwrap(),
        ));

        let c = p.color_at_object(&o, Point::new(1.5, 0.0, 0.0));

        assert_eq!(c, color::consts::WHITE);
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let o = Shape::Sphere(Sphere::from(SphereBuilder {
            transform: Transform::scaling(2.0, 2.0, 2.0).unwrap(),
            ..Default::default()
        }));

        let p = Pattern3D::Stripe(Schema::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Transform::translation(0.5, 0.0, 0.0),
        ));

        let c = p.color_at_object(&o, Point::new(2.5, 0.0, 0.0));

        assert_eq!(c, color::consts::WHITE);
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let o = Shape::Sphere(Sphere::from(SphereBuilder {
            transform: Transform::scaling(2.0, 2.0, 2.0).unwrap(),
            ..Default::default()
        }));

        let p = TestPattern::default();

        let c = p.color_at_object(&o, Point::new(2.0, 3.0, 4.0));

        assert_eq!(
            c,
            Color {
                red: 1.0,
                green: 1.5,
                blue: 2.0
            }
        );
    }

    #[test]
    fn a_pattern_with_a_pattern_transformation() {
        let o = Shape::Sphere(Default::default());

        let p = TestPattern(Schema::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Transform::scaling(2.0, 2.0, 2.0).unwrap(),
        ));

        let c = p.color_at_object(&o, Point::new(2.0, 3.0, 4.0));

        assert_eq!(
            c,
            Color {
                red: 1.0,
                green: 1.5,
                blue: 2.0
            }
        );
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let o = Shape::Sphere(Sphere::from(SphereBuilder {
            transform: Transform::scaling(2.0, 2.0, 2.0).unwrap(),
            ..Default::default()
        }));

        let p = TestPattern(Schema::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Transform::translation(0.5, 1.0, 1.5),
        ));

        let c = p.color_at_object(&o, Point::new(2.5, 3.0, 3.5));

        assert_eq!(
            c,
            Color {
                red: 0.75,
                green: 0.5,
                blue: 0.25
            }
        );
    }

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let p = Pattern3D::Gradient(Schema::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Default::default(),
        ));

        assert_eq!(p.color_at(Point::new(0.0, 0.0, 0.0)), color::consts::WHITE);
        assert_eq!(
            p.color_at(Point::new(0.25, 0.0, 0.0)),
            Color {
                red: 0.75,
                green: 0.75,
                blue: 0.75
            }
        );
        assert_eq!(
            p.color_at(Point::new(0.5, 0.0, 0.0)),
            Color {
                red: 0.5,
                green: 0.5,
                blue: 0.5
            }
        );
        assert_eq!(
            p.color_at(Point::new(0.75, 0.0, 0.0)),
            Color {
                red: 0.25,
                green: 0.25,
                blue: 0.25
            }
        );
    }

    #[test]
    fn a_ring_should_extend_in_both_x_and_z() {
        let p = Pattern3D::Ring(Schema::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Default::default(),
        ));

        assert_eq!(p.color_at(Point::new(0.0, 0.0, 0.0)), color::consts::WHITE);
        assert_eq!(p.color_at(Point::new(1.0, 0.0, 0.0)), color::consts::BLACK);
        assert_eq!(p.color_at(Point::new(0.0, 0.0, 1.0)), color::consts::BLACK);
        assert_eq!(
            p.color_at(Point::new(0.708, 0.0, 0.708)),
            color::consts::BLACK
        );
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let p = Pattern3D::Checker(Schema::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Default::default(),
        ));

        assert_eq!(p.color_at(Point::new(0.0, 0.0, 0.0)), color::consts::WHITE);
        assert_eq!(p.color_at(Point::new(0.99, 0.0, 0.0)), color::consts::WHITE);
        assert_eq!(p.color_at(Point::new(1.01, 0.0, 0.0)), color::consts::BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let p = Pattern3D::Checker(Schema::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Default::default(),
        ));

        assert_eq!(p.color_at(Point::new(0.0, 0.0, 0.0)), color::consts::WHITE);
        assert_eq!(p.color_at(Point::new(0.0, 0.99, 0.0)), color::consts::WHITE);
        assert_eq!(p.color_at(Point::new(0.0, 1.01, 0.0)), color::consts::BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let p = Pattern3D::Checker(Schema::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Default::default(),
        ));

        assert_eq!(p.color_at(Point::new(0.0, 0.0, 0.0)), color::consts::WHITE);
        assert_eq!(p.color_at(Point::new(0.0, 0.0, 0.99)), color::consts::WHITE);
        assert_eq!(p.color_at(Point::new(0.0, 0.0, 1.01)), color::consts::BLACK);
    }
}
