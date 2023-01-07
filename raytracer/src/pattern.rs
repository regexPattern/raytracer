use crate::{
    color::Color,
    float,
    shape::Object,
    transform::Transform,
    tuple::{Point, Tuple},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Texture3D {
    pub a: Color,
    pub b: Color,
    pub transform: Transform,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern3D {
    Solid(Color),
    Stripe(Texture3D),
    Gradient(Texture3D),
    Ring(Texture3D),
    Checker(Texture3D),
}

fn pattern_point(object: &Object, pattern_transform: Transform, world_point: Point) -> Point {
    let object_point = object.transform.inverse() * world_point;

    pattern_transform.inverse() * object_point
}

impl Texture3D {
    pub fn new(a: Color, b: Color) -> Self {
        let transform = Transform::default();

        Self { a, b, transform }
    }
}

impl Pattern3D {
    pub(crate) fn color_at_object(&self, object: &Object, world_point: Point) -> Color {
        self.color_at(pattern_point(object, self.transform(), world_point))
    }

    fn color_at(&self, pattern_point: Point) -> Color {
        let Point(Tuple { x, y, z, .. }) = pattern_point;

        match self {
            Self::Solid(c) => c.to_owned(),
            Self::Stripe(s) => {
                if float::approx(x.floor() % 2.0, 0.0) {
                    s.a
                } else {
                    s.b
                }
            }
            Self::Gradient(s) => s.a + (s.b - s.a) * (x - x.floor()),
            Self::Ring(s) => {
                if float::approx(x.hypot(z).floor() % 2.0, 0.0) {
                    s.a
                } else {
                    s.b
                }
            }
            Self::Checker(s) => {
                if float::approx((x.floor() + y.floor() + z.floor()) % 2.0, 0.0) {
                    s.a
                } else {
                    s.b
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

#[cfg(test)]
mod tests {
    use crate::color::consts::{BLACK, WHITE};

    use super::*;

    #[derive(Debug)]
    struct TestPattern(Texture3D);

    impl Default for TestPattern {
        fn default() -> Self {
            Self(Texture3D {
                a: WHITE,
                b: BLACK,
                transform: Default::default(),
            })
        }
    }

    impl TestPattern {
        fn color_at_object(&self, object: &Object, point: Point) -> Color {
            let pattern_point = pattern_point(object, self.0.transform, point);

            Color {
                red: pattern_point.0.x,
                green: pattern_point.0.y,
                blue: pattern_point.0.z,
            }
        }
    }

    #[test]
    fn creating_a_stripe_pattern() {
        let p = Pattern3D::Stripe(Texture3D::new(WHITE, BLACK));

        assert!(matches!(p, Pattern3D::Stripe(Texture3D { a, .. }) if a == WHITE));
        assert!(matches!(p, Pattern3D::Stripe(Texture3D { b, .. }) if b == BLACK));
        assert!(
            matches!(p, Pattern3D::Stripe(Texture3D { transform: t, .. }) if t == Default::default())
        );
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let p = Pattern3D::Stripe(Texture3D::new(WHITE, BLACK));

        assert_eq!(p.color_at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.color_at(Point::new(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(p.color_at(Point::new(0.0, 2.0, 0.0)), WHITE);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let p = Pattern3D::Stripe(Texture3D::new(WHITE, BLACK));

        assert_eq!(p.color_at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.color_at(Point::new(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(p.color_at(Point::new(0.0, 0.0, 2.0)), WHITE);
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let p = Pattern3D::Stripe(Texture3D::new(WHITE, BLACK));

        assert_eq!(p.color_at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.color_at(Point::new(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(p.color_at(Point::new(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(p.color_at(Point::new(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(p.color_at(Point::new(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(p.color_at(Point::new(-1.1, 0.0, 0.0)), WHITE);
    }

    #[test]
    fn stripes_with_object_transform() {
        let o = Object {
            transform: Transform::try_scaling(2.0, 2.0, 2.0).unwrap(),
            ..Default::default()
        };

        let p = Pattern3D::Stripe(Texture3D::new(WHITE, BLACK));

        let c = p.color_at_object(&o, Point::new(1.5, 0.0, 0.0));

        assert_eq!(c, WHITE);
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let o = Object::default();

        let p = Pattern3D::Stripe(Texture3D {
            a: WHITE,
            b: BLACK,
            transform: Transform::try_scaling(2.0, 2.0, 2.0).unwrap(),
        });

        let c = p.color_at_object(&o, Point::new(1.5, 0.0, 0.0));

        assert_eq!(c, WHITE);
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let o = Object {
            transform: Transform::try_scaling(2.0, 2.0, 2.0).unwrap(),
            ..Default::default()
        };

        let p = Pattern3D::Stripe(Texture3D {
            a: WHITE,
            b: BLACK,
            transform: Transform::translation(0.5, 0.0, 0.0),
        });

        let c = p.color_at_object(&o, Point::new(2.5, 0.0, 0.0));

        assert_eq!(c, WHITE);
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let o = Object {
            transform: Transform::try_scaling(2.0, 2.0, 2.0).unwrap(),
            ..Default::default()
        };

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
        let o = Object::default();

        let p = TestPattern(Texture3D {
            a: WHITE,
            b: BLACK,
            transform: Transform::try_scaling(2.0, 2.0, 2.0).unwrap(),
        });

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
        let o = Object {
            transform: Transform::try_scaling(2.0, 2.0, 2.0).unwrap(),
            ..Default::default()
        };

        let p = TestPattern(Texture3D {
            a: WHITE,
            b: BLACK,
            transform: Transform::translation(0.5, 1.0, 1.5),
        });

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
        let p = Pattern3D::Gradient(Texture3D::new(WHITE, BLACK));

        assert_eq!(p.color_at(Point::new(0.0, 0.0, 0.0)), WHITE);
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
        let p = Pattern3D::Ring(Texture3D::new(WHITE, BLACK));

        assert_eq!(p.color_at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.color_at(Point::new(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(p.color_at(Point::new(0.0, 0.0, 1.0)), BLACK);
        assert_eq!(p.color_at(Point::new(0.708, 0.0, 0.708)), BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let p = Pattern3D::Checker(Texture3D::new(WHITE, BLACK));

        assert_eq!(p.color_at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.color_at(Point::new(0.99, 0.0, 0.0)), WHITE);
        assert_eq!(p.color_at(Point::new(1.01, 0.0, 0.0)), BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let p = Pattern3D::Checker(Texture3D::new(WHITE, BLACK));

        assert_eq!(p.color_at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.color_at(Point::new(0.0, 0.99, 0.0)), WHITE);
        assert_eq!(p.color_at(Point::new(0.0, 1.01, 0.0)), BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let p = Pattern3D::Checker(Texture3D::new(WHITE, BLACK));

        assert_eq!(p.color_at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.color_at(Point::new(0.0, 0.0, 0.99)), WHITE);
        assert_eq!(p.color_at(Point::new(0.0, 0.0, 1.01)), BLACK);
    }

    #[test]
    fn checkers_pattern_in_2d() {
        let p = UVChecker {
            width: 2,
            height: 2,
            a: BLACK,
            b: WHITE,
        };

        assert_eq!(p.color_at(0.0, 0.0), BLACK);
        assert_eq!(p.color_at(0.5, 0.0), WHITE);
        assert_eq!(p.color_at(0.0, 0.5), WHITE);
        assert_eq!(p.color_at(0.5, 0.0), WHITE);
        assert_eq!(p.color_at(0.5, 0.5), BLACK);
        assert_eq!(p.color_at(1.0, 1.0), BLACK);
    }
}
