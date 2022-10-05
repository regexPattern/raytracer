pub const EPSILON: f64 = 0.00001;

pub fn approx(lhs: f64, rhs: f64) -> bool {
    (lhs - rhs).abs() < EPSILON
}

pub fn assert_approx(a: f64, b: f64) {
    if !approx(a, b) {
        panic!(
            "floats are not approximately nor actually equal: `(left !~= right)`
left: `{}`,
right: `{}`,
delta: `{}`,
epsilon: `{}`",
            a,
            b,
            (a - b).abs(),
            crate::float::EPSILON
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparing_approximate_floats() {
        assert_approx(1.0, 1.0);
        assert_approx(3.14159, std::f64::consts::PI);
        assert_approx(-2.0, -2.0);
        assert_approx(1.0, 1.0 + 0.000009);
    }

    #[test]
    #[should_panic]
    fn comparing_floats_with_non_approximate_whole_parts() {
        assert_approx(-1.0, 0.0);
    }

    #[test]
    #[should_panic]
    fn comparing_floats_with_non_approximate_decimal_parts() {
        assert_approx(3.1415, std::f64::consts::PI);
    }

    #[test]
    #[should_panic]
    fn comparing_floats_which_difference_is_equal_to_epsilon() {
        assert_approx(1.0, 1.0 + EPSILON);
    }
}
