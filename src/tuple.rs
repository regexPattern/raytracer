mod point;
mod vector;
mod color;

use std::cmp::{Ordering, PartialOrd};
use std::ops::{Add, Div, Mul, Neg, Sub};

pub use crate::tuple::point::Point;
pub use crate::tuple::vector::Vector;

// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
#[derive(Copy, Clone, Debug)]
pub struct Scalar(pub f64);

impl PartialEq for Scalar {
    fn eq(&self, other: &Scalar) -> bool {
        (self.0 - other.0).abs() < f64::EPSILON
    }
}

impl PartialOrd for Scalar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.total_cmp(&other.0))
    }
}

impl Add for Scalar {
    type Output = Scalar;

    fn add(self, rhs: Scalar) -> Scalar {
        Scalar(self.0 + rhs.0)
    }
}

impl Sub for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: Scalar) -> Scalar {
        Scalar(self.0 - rhs.0)
    }
}

impl Mul for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: Self) -> Scalar {
        Scalar(self.0 * rhs.0)
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

    // TODO: Me gustaria usar el trait `Into` aca.
    fn coordinates(&self) -> (f64, f64, f64) {
        (self.x.0, self.y.0, self.z.0)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Tuple {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;

        Tuple::new(x.0, y.0, z.0)
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

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple::new(0.0, 0.0, 0.0) - self
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

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Tuple {
        let (x, y, z) = self.coordinates();

        Tuple::new(x / rhs, y / rhs, z / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparing_two_scalar() {
        let c1 = Scalar(1.0);
        let c2 = Scalar(1.0);
        let c3 = Scalar(1.0 + f64::EPSILON);

        assert_eq!(c1, c2);
        assert_ne!(c1, c3);

        let c4 = Scalar(1.0);
        let c5 = Scalar(2.0);

        assert!(c4 <= c4);
        assert!(c4 >= c4);
        assert!(c4 < c5);
        assert!(c5 > c4);
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
