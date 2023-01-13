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
            $crate::float::approx($a, $b),
            "assertion failed: `(left == right)`
    left: `{:.5}`,
   right: `{:.5}`
   delta: `{:.5}`
 epsilon: `{:.5}`",
            $a,
            $b,
            f64::from($a - $b).abs(),
            $crate::float::EPSILON
        );
    }};
}

#[cfg(test)]
mod tests {
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
}
