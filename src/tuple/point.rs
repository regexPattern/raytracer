use crate::tuple::{Vector, Scalar, Tuple};

use std::ops::{Add, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub tuple: Tuple,
    w: Scalar,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            tuple: Tuple::new(x, y, z),
            w: Scalar(1.0),
        }
    }
}

impl From<Tuple> for Point {
    fn from(tuple: Tuple) -> Point {
        let (x, y, z) = tuple.coordinates();

        Point::new(x, y, z)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.tuple == other.tuple && self.w == other.w
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Point {
        Point::from(self.tuple + rhs.tuple)
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Point {
        Point::from(self.tuple * rhs)
    }
}

impl Mul<Scalar> for Point {
    type Output = Point;

    fn mul(self, rhs: Scalar) -> Point {
        // TODO: Investigar como es que `rhs.0` no es privado???
        self * rhs.0
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point::from(-self.tuple)
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Vector {
        Vector::from(self.tuple - rhs.tuple)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Point {
        Point::from(self.tuple - rhs.tuple)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_point() {
        let p = Point::new(1.0, 2.0, 3.0);

        assert_eq!(p.tuple, Tuple::new(1.0, 2.0, 3.0));
        assert_eq!(p.w, 1.0);
    }

    #[test]
    fn creating_point_from_tuple() {
        let p = Point::from(Tuple::new(1.0, 2.0, 3.0));

        assert_eq!(p, Point::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn comparing_two_points() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(1.0, 2.0, 3.0);
        let p3 = Point::new(2.0, 2.0, 3.0);

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

    #[test]
    fn adding_vector_to_point() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(p + v, Point::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn multiplying_point_with_float() {
        let p = Point::new(1.0, 2.0, 3.0);

        assert_eq!(p * 2.0, Point::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn multiplying_point_with_scalar() {
        let p = Point::new(1.0, 2.0, 3.0);

        assert_eq!(p * Scalar(2.0), Point::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn negating_point() {
        let p = Point::new(1.0, 2.0, 3.0);

        assert_eq!(-p, Point::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(p - v, Point::new(-2.0, -4.0, -6.0));
    }
}
