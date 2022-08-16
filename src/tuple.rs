mod color;
mod point;
mod vector;

use std::ops::{Add, Div, Mul, Neg, Sub};

pub use crate::tuple::color::Color;
pub use crate::tuple::point::Point;
pub use crate::tuple::vector::Vector;

#[derive(Copy, Clone, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Tuple {
    fn new(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z }
    }
}

impl From<[f64; 3]> for Tuple {
    fn from(array: [f64; 3]) -> Tuple {
        Tuple::new(array[0], array[1], array[2])
    }
}

impl IntoIterator for Tuple {
    type Item = f64;
    type IntoIter = std::array::IntoIter<Self::Item, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.x, self.y, self.z].into_iter()
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        (self.x - other.x).abs() < f64::EPSILON
            && (self.y - other.y).abs() < f64::EPSILON
            && (self.z - other.z).abs() < f64::EPSILON
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;

        Tuple::new(x, y, z)
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Mul for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;

        Tuple::new(x, y, z)
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple::new(0.0, 0.0, 0.0) - self
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;

        Tuple::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
