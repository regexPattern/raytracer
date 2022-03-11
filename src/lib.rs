#![allow(dead_code)]

use std::cmp::PartialEq;
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

fn compare_floating_points(a: f64, b: f64) -> bool {
    const EPSILON: f64 = 0.00001;
    a.abs() - b.abs() < EPSILON
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        compare_floating_points(self.x, other.x)
            && compare_floating_points(self.y, other.y)
            && compare_floating_points(self.z, other.z)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        compare_floating_points(self.x, other.x)
            && compare_floating_points(self.y, other.y)
            && compare_floating_points(self.z, other.z)
    }
}

// Point + Vector -> Point
impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// Vector + Point -> Point
impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// Vector + Vector -> Vector
impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// Point - Point -> Vector
impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// Point - Vector -> Point
impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, other: Vector) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// Vector - Vector -> Vector
impl Sub<Vector> for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, factor: f64) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: 0.0 - self.x,
            y: 0.0 - self.y,
            z: 0.0 - self.z,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, factor: f64) -> Self {
        Self {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_are_created_desired_values() {
        let point = Point::new(4.0, -4.0, 3.0);
        assert_eq!(point.x, 4.0);
        assert_eq!(point.y, -4.0);
        assert_eq!(point.z, 3.0);
    }

    #[test]
    fn vectors_are_created_desired_values() {
        let vector = Vector::new(4.0, -4.0, 3.0);
        assert_eq!(vector.x, 4.0);
        assert_eq!(vector.y, -4.0);
        assert_eq!(vector.z, 3.0);
    }

    #[test]
    fn adding_point_and_vector() {
        let point = Point::new(3.0, 2.0, 1.0);
        let vector = Vector::new(5.0, 6.0, 7.0);
        let expected = Point::new(8.0, 10.0, 8.0);

        assert_eq!(point + vector, expected);
    }

    #[test]
    fn adding_vector_and_point() {
        let point = Point::new(3.0, 2.0, 1.0);
        let vector = Vector::new(5.0, 6.0, 7.0);
        let expected = Point::new(8.0, 10.0, 8.0);

        assert_eq!(vector + point, expected);
    }

    #[test]
    fn adding_two_vectors() {
        let vector1 = Vector::new(3.0, -2.0, 5.0);
        let vector2 = Vector::new(-2.0, 3.0, 1.0);
        let expected = Vector::new(1.0, 1.0, 6.0);

        assert_eq!(vector1 + vector2, expected);
    }

    #[test]
    fn subtracting_two_points() {
        let point1 = Point::new(3.0, 2.0, 1.0);
        let point2 = Point::new(5.0, 6.0, 7.0);
        let expected = Vector::new(-2.0, -4.0, -6.0);

        assert_eq!(point1 - point2, expected);
    }

    #[test]
    fn subtracting_vector_from_point() {
        let point = Point::new(3.0, 2.0, 1.0);
        let vector = Vector::new(5.0, 6.0, 7.0);
        let expected = Point::new(-2.0, -4.0, -6.0);

        assert_eq!(point - vector, expected);
    }

    #[test]
    fn subtracting_two_vectors() {
        let vector1 = Vector::new(3.0, 2.0, 1.0);
        let vector2 = Vector::new(5.0, 6.0, 7.0);
        let expected = Vector::new(-2.0, -4.0, -6.0);

        assert_eq!(vector1 - vector2, expected);
    }

    #[test]
    fn negating_vector() {
        let vector = Vector::new(1.0, -2.0, 3.0);
        let expected = Vector::new(-1.0, 2.0, -3.0);

        assert_eq!(-vector, expected);
    }

    #[test]
    fn multiply_vector_by_scalar() {
        let vector = Vector::new(1.0, -2.0, 3.0);
        let expected = Vector::new(3.5, -7.0, 10.5);

        assert_eq!(vector * 3.5, expected);
    }

    #[test]
    fn dividing_vector_by_scalar() {
        let vector = Vector::new(1.0, -2.0, 3.0);
        let expected = Vector::new(0.5, -1.0, 1.5);

        assert_eq!(vector / 2.0, expected);
    }
}
