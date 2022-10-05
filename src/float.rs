use std::ops::{Add, Div, Mul, Sub};

const EPSILON: f64 = 0.00001;

#[derive(Copy, Clone, Debug)]
pub struct Float(f64);

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < EPSILON
    }
}

impl PartialEq<f64> for Float {
    fn eq(&self, other: &f64) -> bool {
        *self == Float(*other)
    }
}

impl PartialEq<Float> for f64 {
    fn eq(&self, other: &Float) -> bool {
        other == self
    }
}

impl Add for Float {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<f64> for Float {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        self + Self(rhs)
    }
}

impl Add<Float> for f64 {
    type Output = Float;

    fn add(self, rhs: Float) -> Self::Output {
        Float(self) + rhs
    }
}

impl Sub for Float {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Sub<f64> for Float {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        self - Float(rhs)
    }
}

impl Sub<Float> for f64 {
    type Output = Float;

    fn sub(self, rhs: Float) -> Self::Output {
        Float(self) - rhs
    }
}

impl Mul for Float {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul<f64> for Float {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self * Self(rhs)
    }
}

impl Mul<Float> for f64 {
    type Output = Float;

    fn mul(self, rhs: Float) -> Self::Output {
        Float(self) * rhs
    }
}

impl Div for Float {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Div<f64> for Float {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Div<Float> for f64 {
    type Output = Float;

    fn div(self, rhs: Float) -> Self::Output {
        Float(self) / rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparing_floats() {
        let f = Float(3.14159);

        assert_eq!(f, Float(3.14159));
        assert_ne!(f, Float(3.0));
    }

    #[test]
    fn comparing_float_and_f64() {
        let f = Float(3.14159);

        assert_eq!(f, 3.14159);
        assert_ne!(f, 3.0);
    }

    #[test]
    fn comparing_f64_and_float() {
        let f = Float(3.14159);

        assert_eq!(3.14159, f);
        assert_ne!(3.0, f);
    }

    #[test]
    fn comparing_floats_within_epsilon() {
        let f = Float(3.14159);

        assert_eq!(f, Float(3.14159 + 0.000009));
        assert_ne!(f, Float(3.14159 + 0.00001));
    }

    #[test]
    fn adding_floats() {
        let f1 = Float(1.0);
        let f2 = Float(2.0);
        let f3 = Float(9.1);

        assert_eq!(f1 + f2, 3.0);
        assert_eq!(f1 + f2, f2 + f1, "`Float` addition is commutative");
        assert_eq!(
            f1 + (f2 + f3),
            (f1 + f2) + f3,
            "`Float` addition is associative"
        );
    }

    #[test]
    fn adding_float_and_f64() {
        let f = Float(1.0);

        assert_eq!(f + 2.0, 3.0);
        assert_eq!(
            f + 2.0,
            2.0 + f,
            "`Float` and `f64` addition is commutative"
        );
    }

    #[test]
    fn subtracting_floats() {
        let f1 = Float(4.0);
        let f2 = Float(2.5);

        assert_eq!(f1 - f2, Float(1.5));
        assert_eq!(f2 - f1, Float(-1.5));
    }

    #[test]
    fn subtracting_float_and_f64() {
        let f = Float(4.5);

        assert_eq!(f - 4.0, 0.5);
        assert_eq!(4.0 - f, -0.5);
    }

    #[test]
    fn multiplying_floats() {
        let f1 = Float(7.1);
        let f2 = Float(3.4);

        assert_eq!(f1 * f2, (7.1 * 3.4));
        assert_eq!(f2 * f1, f1 * f2, "`Float` multiplication is commutative");
    }

    #[test]
    fn multiplying_float_and_f64() {
        let f = Float(3.45);

        assert_eq!(f * 7.11, (3.45 * 7.11));
        assert_eq!(
            f * 7.11,
            7.11 * f,
            "`Float` and `f64` multiplication is commutative"
        );
    }

    #[test]
    fn float_multiplication_is_distributive() {
        let f1 = Float(2.0);
        let f2 = Float(3.1);

        assert_eq!(4.5 * (f1 + f2), 4.5 * f1 + 4.5 * f2);
    }

    #[test]
    fn dividing_floats() {
        let f1 = Float(7.5);
        let f2 = Float(2.0);

        assert_eq!(f1 / f2, 3.75);
        assert!(
            (f1 / Float(0.0)).0.is_infinite(),
            "`Float` division by zero returns infinite"
        );
    }

    #[test]
    fn dividing_float_and_f64() {
        let f = Float(7.5);

        assert_eq!(f / 2.0, 3.75);
        assert_eq!(7.5 / Float(2.0), 3.75);
    }
}
