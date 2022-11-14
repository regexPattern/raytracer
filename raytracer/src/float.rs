pub const EPSILON: f64 = 0.00001;

pub fn approx(lhs: f64, rhs: f64) -> bool {
    (lhs - rhs).abs() < EPSILON
}

#[macro_export]
macro_rules! assert_approx {
    ( $a:expr, $b:expr ) => {{
        $crate::assert_approx!($a, $b, "")
    }};
    ( $a:expr, $b:expr, $desc:expr ) => {{
        let (a, b) = ($a as f64, $b as f64);
        let desc: &'static str = $desc;
        let desc = if desc.len() > 0 {
            format!("\ndesc: '{}'", desc)
        } else {
            String::new()
        };

        if !$crate::float::approx(a, b) {
            panic!(
                "floats are not approximately nor actually equal: `(left == right)`
left: `{:.5}`,
right: `{:.5}`,
delta: `{:.5}`,
epsilon: `{:.5}`{}",
                a,
                b,
                (a - b).abs(),
                $crate::float::EPSILON,
                desc
            )
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparing_approximate_floats() {
        assert_approx!(1.0, 1.0);
        assert_approx!(3.14159, std::f64::consts::PI);
        assert_approx!(-2.0, -2.0);
        assert_approx!(1.0, 1.0 + 0.000009);
    }

    #[test]
    #[should_panic(
        expected = "floats are not approximately nor actually equal: `(left == right)`
left: `-1.00000`,
right: `0.00000`,
delta: `1.00000`,
epsilon: `0.00001`"
    )]
    fn comparing_floats_with_non_approximate_whole_parts() {
        assert_approx!(-1.0, 0.0);
    }

    #[test]
    #[should_panic(
        expected = "floats are not approximately nor actually equal: `(left == right)`
left: `3.14150`,
right: `3.14159`,
delta: `0.00009`,
epsilon: `0.00001`"
    )]
    fn comparing_floats_with_non_approximate_decimal_parts() {
        assert_approx!(3.1415, std::f64::consts::PI);
    }

    #[test]
    #[should_panic(
        expected = "floats are not approximately nor actually equal: `(left == right)`
left: `1.00000`,
right: `1.00001`,
delta: `0.00001`,
epsilon: `0.00001`"
    )]
    fn comparing_floats_which_difference_is_equal_to_epsilon() {
        assert_approx!(1.0, 1.0 + EPSILON);
    }

    #[test]
    #[should_panic(
        expected = "floats are not approximately nor actually equal: `(left == right)`
left: `1.00000`,
right: `2.00000`,
delta: `1.00000`,
epsilon: `0.00001`
desc: 'one is equal to two'"
    )]
    fn approx_macro_with_description_message() {
        assert_approx!(1.0, 2.0, "one is equal to two");
    }
}
