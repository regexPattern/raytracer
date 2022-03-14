#![allow(dead_code)]
use std::ops::{Add, Mul, Neg, Sub};

use crate::tuple::Tuple;

#[derive(Copy, Clone, Debug)]
struct Coordinate {
    tuple: Tuple,
    w: f64,
}

impl Coordinate {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Coordinate {
            tuple: Tuple::new(x, y, z),
            w,
        }
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.tuple == other.tuple && (self.w - other.w).abs() <= f64::EPSILON
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Coordinate {
            tuple: self.tuple + other.tuple,
            w: self.w + other.w,
        }
    }
}

impl Neg for Coordinate {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Coordinate {
            tuple: -self.tuple,
            w: -self.w,
        }
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + -other
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point(Coordinate);

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point(Coordinate::new(x, y, z, 1.))
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self::Output {
        Point(self.0 + other.0)
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, factor: f64) -> Self::Output {
        Point(Coordinate {
            tuple: self.0.tuple * factor,
            w: self.0.w,
        })
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point(Coordinate {
            tuple: -self.0.tuple,
            w: self.0.w,
        })
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Self::Output {
        Vector(self.0 - other.0)
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, other: Vector) -> Self::Output {
        Point(self.0 - other.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Vector(Coordinate);

impl Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector(Coordinate::new(x, y, z, 0.))
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector(self.0 + other.0)
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point(self.0 + other.0)
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, factor: f64) -> Self::Output {
        Vector(self.0 * factor)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector(-self.0)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector(self.0 - other.0)
    }
}

#[cfg(test)]
mod create {
    use super::*;

    #[test]
    fn creating_point() {
        let p = Point::new(1., 2., 3.);
        let c = Coordinate::new(1., 2., 3., 1.);

        assert_eq!(p.0, c);
    }

    #[test]
    fn creating_vector() {
        let v = Vector::new(1., 2., 3.);
        let c = Coordinate::new(1., 2., 3., 0.);

        assert_eq!(v.0, c);
    }

    #[test]
    fn comparing_points() {
        let p = Point::new(1., 2., 3.);

        assert_eq!(p, Point::new(1., 2., 3.));
        assert_eq!(p, Point::new(1. + f64::EPSILON, 2., 3.));
        assert_ne!(
            p,
            Point::new(1. + (f64::EPSILON * 2.), 2., 3.),
            "Point comparision falls between the f64::EPSILON range"
        );
    }

    #[test]
    fn comparing_vectors() {
        let v = Vector::new(1., 2., 3.);

        assert_eq!(v, Vector::new(1., 2., 3.));
        assert_eq!(v, Vector::new(1. + f64::EPSILON, 2., 3.));
        assert_ne!(
            v,
            Vector::new(1. + (f64::EPSILON * 2.), 2., 3.),
            "Vector comparision falls between the f64::EPSILON range"
        );
    }
}

#[cfg(test)]
mod ops {
    use super::*;

    #[test]
    fn adding_vector_to_point() {
        let p = Point::new(3., -2., 5.);
        let v = Vector::new(-2., 3., 1.);

        assert_eq!(p + v, Point::new(1., 1., 6.));
    }

    #[test]
    fn adding_vector_to_vector() {
        let v1 = Vector::new(3., -2., 5.);
        let v2 = Vector::new(-2., 3., 1.);

        assert_eq!(v1 + v2, Vector::new(1., 1., 6.));
    }

    #[test]
    fn adding_point_to_vector() {
        let v = Vector::new(-2., 3., 1.);
        let p = Point::new(3., -2., 5.);

        assert_eq!(v + p, Point::new(1., 1., 6.));
    }

    #[test]
    fn subtracting_point_from_point() {
        let p1 = Point::new(-2., 3., 1.);
        let p2 = Point::new(3., -2., 5.);

        assert_eq!(p1 - p2, Vector::new(-5., 5., -4.));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = Point::new(3., -2., 5.);
        let v = Vector::new(-2., 3., 1.);

        assert_eq!(p - v, Point::new(5., -5., 4.));
    }

    #[test]
    fn subtracting_vector_from_vector() {
        let v1 = Vector::new(-2., 3., 1.);
        let v2 = Vector::new(3., -2., 5.);

        assert_eq!(v1 - v2, Vector::new(-5., 5., -4.));
    }

    #[test]
    fn negating_point() {
        let p = Point::new(1., -2., 3.);

        assert_eq!(-p, Point::new(-1., 2., -3.));
    }

    #[test]
    fn negating_vector() {
        let v = Vector::new(1., -2., 3.);

        assert_eq!(-v, Vector::new(-1., 2., -3.));
    }

    #[test]
    fn multiplying_point_by_scalar() {
        let p = Point::new(1., -2., 3.);

        assert_eq!(p * 3.5, Point::new(3.5, -7., 10.5));
        assert_eq!(p * 0.5, Point::new(0.5, -1., 1.5));
        assert_eq!(p * -2., Point::new(-2., 4., -6.));
    }

    #[test]
    fn multiplying_vector_by_scalar() {
        let v = Vector::new(1., -2., 3.);

        assert_eq!(v * 3.5, Vector::new(3.5, -7., 10.5));
        assert_eq!(v * 0.5, Vector::new(0.5, -1., 1.5));
        assert_eq!(v * -2., Vector::new(-2., 4., -6.));
    }
}
