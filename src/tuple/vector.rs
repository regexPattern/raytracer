use crate::tuple::{Point, Scalar, Tuple};

use std::ops::{Add, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub tuple: Tuple,
    w: Scalar,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector {
            tuple: Tuple::new(x, y, z),
            w: Scalar(0.0),
        }
    }

    pub fn magnitude(&self) -> Scalar {
        let magnitude = self.tuple
            .into_iter()
            .fold(0.0, |sum, n| sum + n.0.powi(2))
            .sqrt();

        Scalar(magnitude)
    }

    pub fn normalize(self) -> Vector {
        let magnitude = self.magnitude();
        match magnitude {
            x if x == Scalar(0.0) => Vector::new(0.0, 0.0, 0.0),
            _ => Vector::from(self.tuple / self.magnitude().0),
        }
    }

    pub fn dot(self, rhs: Vector) -> Scalar {
        self.tuple
            .into_iter()
            .zip(rhs.tuple.into_iter())
            .fold(Scalar(0.0), |sum, (a, b)| sum + (a * b))
    }

    pub fn cross(self, rhs: Vector) -> Vector {
        let x = self.tuple.y * rhs.tuple.z - self.tuple.z * rhs.tuple.y;
        let y = self.tuple.z * rhs.tuple.x - self.tuple.x * rhs.tuple.z;
        let z = self.tuple.x * rhs.tuple.y - self.tuple.y * rhs.tuple.x;

        Vector::from(Tuple::new(x.0, y.0, z.0))
    }
}

impl From<Tuple> for Vector {
    fn from(tuple: Tuple) -> Vector {
        let (x, y, z) = tuple.coordinates();

        Vector::new(x, y, z)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.tuple == other.tuple && self.w == other.w
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector::from(self.tuple + rhs.tuple)
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point::from(self.tuple + rhs.tuple)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        Vector::from(self.tuple * rhs)
    }
}

impl Mul<Scalar> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Scalar) -> Vector {
        self * rhs.0
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector::from(-self.tuple)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector::from(self.tuple - rhs.tuple)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v.tuple, Tuple::new(1.0, 2.0, 3.0));
        assert_eq!(v.w, 0.0);
    }

    #[test]
    fn creating_vector_from_tuple() {
        let v = Vector::from(Tuple::new(1.0, 2.0, 3.0));

        assert_eq!(v, Vector::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn comparing_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(1.0, 2.0, 3.0);
        let v3 = Vector::new(2.0, 2.0, 3.0);

        assert_eq!(v1, v2);
        assert_ne!(v2, v3);
    }

    #[test]
    fn adding_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(v1 + v2, Vector::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn adding_point_to_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(v + p, Point::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn multiplying_vector_with_float() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v * 2.0, Vector::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn multiplying_vector_with_scalar() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v * Scalar(2.0), Vector::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn negating_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(-v, Vector::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(v1 - v2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn vector_magnitude() {
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 1.0, 0.0);
        let v3 = Vector::new(0.0, 0.0, 1.0);
        let v4 = Vector::new(1.0, 2.0, 3.0);
        let v5 = Vector::new(0.0, 0.0, 0.0);

        assert_eq!(v1.magnitude(), 1.0);
        assert_eq!(v2.magnitude(), 1.0);
        assert_eq!(v3.magnitude(), 1.0);
        assert_eq!(v4.magnitude(), 14_f64.sqrt());
        assert_eq!(v5.magnitude(), 0.0);
    }

    #[test]
    fn vector_normalization() {
        let v1 = Vector::new(4.0, 0.0, 0.0);
        let v2 = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v1.normalize(), Vector::new(1.0, 0.0, 0.0));
        assert_eq!(
            v2.normalize(),
            Vector::new(
                1.0 / 14_f64.sqrt(),
                2.0 / 14_f64.sqrt(),
                3.0 / 14_f64.sqrt()
            )
        );
    }

    #[test]
    fn normalize_null_vector() {
        let v = Vector::new(0.0, 0.0, 0.0);

        assert_eq!(v.normalize(), Vector::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn normalized_vector_magnitude() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v.normalize().magnitude(), 1.0);
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(v1.dot(v2), 20.0);
        assert_eq!(v2.dot(v1), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(v1.cross(v2), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(v1), Vector::new(1.0, -2.0, 1.0));
    }
}