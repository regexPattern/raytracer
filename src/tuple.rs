mod color;
mod point;
mod vector;

use std::cmp::{Ordering, PartialOrd};
use std::ops::{Add, Div, Mul, Neg, Sub};

pub use crate::tuple::point::Point;
pub use crate::tuple::vector::Vector;

#[derive(Copy, Clone, Debug)]
pub struct Scalar(f64);

impl From<f64> for Scalar {
    fn from(value: f64) -> Scalar {
        Scalar(value)
    }
}

impl PartialEq for Scalar {
    fn eq(&self, other: &Scalar) -> bool {
        (self.0 - other.0).abs() < f64::EPSILON
    }
}

impl PartialEq<f64> for Scalar {
    fn eq(&self, other: &f64) -> bool {
        *self == Scalar(*other)
    }
}

impl PartialOrd for Scalar {
    fn partial_cmp(&self, other: &Scalar) -> Option<Ordering> {
        Some(self.0.total_cmp(&other.0))
    }
}

impl PartialOrd<f64> for Scalar {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        Some(self.0.total_cmp(&other))
    }
}

impl Add for Scalar {
    type Output = Scalar;

    fn add(self, rhs: Scalar) -> Scalar {
        Scalar(self.0 + rhs.0)
    }
}

impl Mul for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: Self) -> Scalar {
        Scalar(self.0 * rhs.0)
    }
}

impl Sub for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: Scalar) -> Scalar {
        Scalar(self.0 - rhs.0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Tuple {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl Tuple {
    fn new(x: f64, y: f64, z: f64) -> Tuple {
        let x = Scalar(x);
        let y = Scalar(y);
        let z = Scalar(z);

        Tuple { x, y, z }
    }

    fn coordinates(&self) -> (f64, f64, f64) {
        (self.x.0, self.y.0, self.z.0)
    }
}

impl From<[f64; 3]> for Tuple {
    fn from(array: [f64; 3]) -> Self {
        Tuple::new(array[0], array[1], array[2])
    }
}

impl IntoIterator for Tuple {
    type Item = Scalar;
    type IntoIter = std::array::IntoIter<Self::Item, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.x, self.y, self.z].into_iter()
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;

        Tuple::new(x.0, y.0, z.0)
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let (x, y, z) = self.coordinates();

        Tuple::new(x / rhs, y / rhs, z / rhs)
    }
}

impl Mul for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;

        Tuple::new(x.0, y.0, z.0)
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Tuple {
        let (x, y, z) = self.coordinates();

        Tuple::new(x * rhs, y * rhs, z * rhs)
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple::new(0.0, 0.0, 0.0) - self
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Tuple {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;

        Tuple::new(x.0, y.0, z.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_scalar_from_float() {
        let s = Scalar::from(1.0);

        assert_eq!(s, Scalar(1.0));
    }

    #[test]
    fn comparing_two_scalars() {
        let s1 = Scalar(1.0);
        let s2 = Scalar(1.0);
        let s3 = Scalar(1.0 + f64::EPSILON);

        assert_eq!(s1, s2);
        assert_ne!(s1, s3);

        let s4 = Scalar(1.0);
        let s5 = Scalar(2.0);

        assert!(s4 <= s4);
        assert!(s4 >= s4);
        assert!(s4 < s5);
        assert!(s5 > s4);
    }

    #[test]
    fn comparing_scalar_with_float() {
        let s1 = Scalar(1.0);
        let s2 = Scalar(1.0 + f64::EPSILON);

        assert_eq!(s1, 1.0);
        assert_ne!(s2, 1.0);

        assert!(s1 < 1.0 + f64::EPSILON);
        assert!(!(s1 > 1.0 + f64::EPSILON));
    }

    #[test]
    fn creating_tuple_from_array() {
        let t = Tuple::from([1.0, 2.0, 3.0]);

        assert_eq!(t, Tuple::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn comparing_two_tuples() {
        let t1 = Tuple::new(1.0, 2.0, 3.0);
        let t2 = Tuple::new(1.0, 2.0, 3.0);
        let t3 = Tuple::new(2.0, 3.0, 4.0);

        assert_eq!(t1, t2);
        assert_ne!(t1, t3);
    }

    #[test]
    fn getting_tuple_coordinates() {
        let t = Tuple::new(1.0, 2.0, 3.0);

        assert_eq!(t.coordinates(), (1.0, 2.0, 3.0));
    }

    #[test]
    fn adding_two_coordiantes() {
        let c1 = Scalar(1.0);
        let c2 = Scalar(2.0);

        assert_eq!(c1 + c2, Scalar(3.0));
    }

    #[test]
    fn adding_two_tuples() {
        let t1 = Tuple::new(1.0, 2.0, 3.0);
        let t2 = Tuple::new(2.0, 3.0, 4.0);

        assert_eq!(t1 + t2, Tuple::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn negating_tuple() {
        let t = Tuple::new(1.0, 2.0, 3.0);

        assert_eq!(-t, Tuple::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn scaling_tuple() {
        let t = Tuple::new(1.0, 2.0, 3.0);

        assert_eq!(t * 2.0, Tuple::new(2.0, 4.0, 6.0));
    }
}
