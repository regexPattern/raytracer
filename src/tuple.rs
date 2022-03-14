use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Tuple(f64, f64, f64);

impl Tuple {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Tuple(a, b, c)
    }
    
    pub fn values(&self) -> [f64; 3] {
        [self.0, self.1, self.2]
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        let diff = *self - *other;
        let diff = [diff.0, diff.1, diff.2];
        !diff.iter().any(|&i| i.abs() > f64::EPSILON)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Tuple::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, factor: f64) -> Self::Output {
        Tuple::new(self.0 / factor, self.1 / factor, self.2 / factor)
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, factor: f64) -> Self::Output {
        Tuple::new(self.0 * factor, self.1 * factor, self.2 * factor)
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple::new(-self.0, -self.1, -self.2)
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Self) -> Self::Output {
        self + -other
    }
}

#[cfg(test)]
mod create {
    use super::*;

    #[test]
    fn creating_tuple() {
        let t = Tuple::new(1., 2., 3.);

        assert_eq!(t.0, 1.);
        assert_eq!(t.1, 2.);
        assert_eq!(t.2, 3.);
    }

    #[test]
    fn comparing_tuples() {
        let t = Tuple::new(1., 2., 3.);

        assert_eq!(t, Tuple::new(1., 2., 3.));
        assert_eq!(
            t,
            Tuple::new(1. + f64::EPSILON, 2., 3.),
            "Tuple comparison falls between the f64::EPSILON range"
        );
        assert_ne!(t, Tuple::new(1. + (f64::EPSILON * 2.), 2., 3.));
    }
}

#[cfg(test)]
mod ops {
    use super::*;

    #[test]
    fn adding_tuples() {
        let t1 = Tuple::new(1., 2., 3.);
        let t2 = Tuple::new(4., 5., 6.);

        assert_eq!(t1 + t2, Tuple::new(5., 7., 9.));
    }

    #[test]
    fn subtracting_tuples() {
        let t1 = Tuple::new(1., 2., 3.);
        let t2 = Tuple::new(4., 5., 6.);

        assert_eq!(t1 - t2, Tuple::new(-3., -3., -3.));
    }

    #[test]
    fn negating_tuple() {
        let t = Tuple::new(1., 2., 3.);

        assert_eq!(-t, Tuple::new(-1., -2., -3.));
    }

    #[test]
    fn multiplying_tuple_by_scalar() {
        let t = Tuple::new(1., -2., 3.);

        assert_eq!(t * 3.5, Tuple::new(3.5, -7., 10.5));
        assert_eq!(t * 0.5, Tuple::new(0.5, -1., 1.5));
        assert_eq!(t * -2., Tuple::new(-2., 4., -6.));
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        let t = Tuple::new(1., -2., 3.);

        assert_eq!(t / 2., Tuple::new(0.5, -1., 1.5));
        assert_eq!(t / 0.5, Tuple::new(2., -4., 6.));
        assert_eq!(t / -1., Tuple::new(-1., 2., -3.));
    }
}
