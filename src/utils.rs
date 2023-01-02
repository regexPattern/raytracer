pub const EPSILON: f64 = 1e-5;

pub fn approx(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

pub fn ge(a: f64, b: f64) -> bool {
    approx(a, b) || a > b
}

pub fn le(a: f64, b: f64) -> bool {
    approx(a, b) || a < b
}

#[macro_export]
macro_rules! assert_approx {
    ($a:expr, $b:expr) => {{
        assert!(
            $crate::utils::approx($a, $b),
            "assertion failed: `(left == right)`
    left: `{:.5}`,
   right: `{:.5}`
   delta: `{:.5}`
 epsilon: `{:.5}`",
            $a,
            $b,
            f64::from($a - $b).abs(),
            $crate::utils::EPSILON
        );
    }};
}

#[cfg(test)]
pub(crate) fn test_world() -> crate::world::World {
    use crate::{
        color,
        light::PointLight,
        material::Material,
        shape::{Object, Shape, Sphere},
        transform::Transform,
        tuple::Point,
        world::World,
    };

    let light = PointLight {
        position: Point::new(-10.0, 10.0, -10.0),
        intensity: color::consts::WHITE,
    };

    let s1 = Shape::Sphere(Sphere(Object {
        material: Material {
            color: color::Color {
                red: 0.8,
                green: 1.0,
                blue: 0.6,
            },
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        },
        ..Default::default()
    }));

    let s2 = Shape::Sphere(Sphere(Object {
        transform: Transform::try_scaling(0.5, 0.5, 0.5).unwrap(),
        ..Default::default()
    }));

    let objects = vec![s1, s2];
    let lights = vec![light];

    World { objects, lights }
}

#[cfg(test)]
mod tests {
    use crate::{
        light::PointLight,
        shape::{Object, Shape, Sphere},
    };

    use super::*;

    #[test]
    fn comparing_two_approximated_floats() {
        let a = 3.14159;
        let b = 3.14159;

        assert_approx!(a, b);
    }

    #[test]
    fn comparing_two_floats_which_difference_is_lower_than_epsilon() {
        let a = 3.14159;
        let b = 3.141595;

        assert_approx!(a, b);
    }

    #[test]
    #[should_panic(expected = "assertion failed: `(left == right)`
    left: `2.71828`,
   right: `3.14159`
   delta: `0.42331`
 epsilon: `0.00001`")]
    fn comparing_two_different_floats_which_difference_is_greater_than_epsilon() {
        let a = std::f64::consts::E;
        let b = std::f64::consts::PI;

        assert_approx!(a, b);
    }

    #[test]
    #[should_panic(expected = "assertion failed: `(left == right)`
    left: `1.00000`,
   right: `1.00001`
   delta: `0.00001`
 epsilon: `0.00001`")]
    fn comparing_two_approximated_floats_which_difference_is_epsilon() {
        let a = 1.0;
        let b = a + EPSILON;

        assert_approx!(a, b);
    }

    #[test]
    fn a_number_is_greater_or_equal_to_other() {
        let a = 1.00001;
        let b = 1.00000;
        let c = 1.00001;

        assert!(ge(a, b));
        assert!(!ge(b, a));
        assert!(ge(a, c));
        assert_eq!(ge(a, c), ge(c, a));
    }

    #[test]
    fn a_number_is_lower_or_equal_to_other() {
        let a = 1.00000;
        let b = 1.00001;
        let c = 1.00000;

        assert!(le(a, b));
        assert!(!le(b, a));
        assert!(le(a, c));
        assert_eq!(le(a, c), le(c, a));
    }

    #[test]
    fn the_default_test_world() {
        let light = PointLight {
            position: crate::tuple::Point::new(-10.0, 10.0, -10.0),
            intensity: crate::color::consts::WHITE,
        };

        let s1 = Shape::Sphere(Sphere(Object {
            material: crate::material::Material {
                color: crate::color::Color {
                    red: 0.8,
                    green: 1.0,
                    blue: 0.6,
                },
                diffuse: 0.7,
                specular: 0.2,
                ..Default::default()
            },
            ..Default::default()
        }));

        let s2 = Shape::Sphere(Sphere(Object {
            transform: crate::transform::Transform::try_scaling(0.5, 0.5, 0.5).unwrap(),
            ..Default::default()
        }));

        let w = test_world();

        assert!(w.lights.contains(&light));
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }
}
