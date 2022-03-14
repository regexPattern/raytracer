use std::ops::{Add, Div, Mul, Neg, Sub};

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

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + -other
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

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, factor: f64) -> Self::Output {
        Point(Coordinate {
            tuple: self.0.tuple / factor,
            w: self.0.w,
        })
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

    fn magnitude(&self) -> f64 {
        let values = self.0.tuple.values();
        values
            .iter()
            .fold(0., |acc, curr| acc + curr.powf(2.))
            .sqrt()
    }

    fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    fn dot(&self, other: &Self) -> f64 {
        let a_values = self.0.tuple.values();
        let b_values = other.0.tuple.values();
        a_values
            .iter()
            .zip(b_values)
            .fold(0., |acc, (a, b)| acc + a * b)
    }

    fn cross(&self, other: &Self) -> Self {
        let [a_x, a_y, a_z] = self.0.tuple.values();
        let [b_x, b_y, b_z] = other.0.tuple.values();

        Vector(Coordinate {
            tuple: Tuple::new(
                a_y * b_z - a_z * b_y,
                a_z * b_x - a_x * b_z,
                a_x * b_y - a_y * b_x
           ),
            w: self.0.w,
        })
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

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, factor: f64) -> Self::Output {
        Vector(Coordinate {
            tuple: self.0.tuple / factor,
            w: self.0.w,
        })
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, factor: f64) -> Self::Output {
        Vector(Coordinate {
            tuple: self.0.tuple * factor,
            w: self.0.w,
        })
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector(Coordinate {
            tuple: -self.0.tuple,
            w: self.0.w,
        })
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
        assert_eq!(p * -0.5, Point::new(-0.5, 1., -1.5));
    }

    #[test]
    fn multiplying_vector_by_scalar() {
        let v = Vector::new(1., -2., 3.);

        assert_eq!(v * 3.5, Vector::new(3.5, -7., 10.5));
        assert_eq!(v * -0.5, Vector::new(-0.5, 1., -1.5));
    }

    #[test]
    fn dividing_point_by_scalar() {
        let p = Point::new(1., -2., 3.);

        assert_eq!(p / 2., Point::new(0.5, -1., 1.5));
        assert_eq!(p / -0.5, Point::new(-2., 4., -6.));
    }

    #[test]
    fn dividing_vector_by_scalar() {
        let v = Vector::new(1., -2., 3.);

        assert_eq!(v / 2., Vector::new(0.5, -1., 1.5));
        assert_eq!(v / -0.5, Vector::new(-2., 4., -6.));
    }

    #[test]
    fn computing_vector_magnitude() {
        let i_hat = Vector::new(1., 0., 0.);
        let j_hat = Vector::new(0., 1., 0.);
        let k_hat = Vector::new(0., 0., 1.);

        assert_eq!(i_hat.magnitude(), 1.);
        assert_eq!(j_hat.magnitude(), 1.);
        assert_eq!(k_hat.magnitude(), 1.);

        let v = Vector::new(1., 2., 3.);

        assert_eq!(v.magnitude(), 14_f64.sqrt());
        assert_eq!((-v).magnitude(), 14_f64.sqrt());
    }

    #[test]
    fn normalizing_vector() {
        let v = Vector::new(4., 0., 0.);

        assert_eq!(v.normalize(), Vector::new(1., 0., 0.));

        let v = Vector::new(1., 2., 3.);

        assert_eq!(
            v.normalize(),
            Vector::new(1. / 14_f64.sqrt(), 2. / 14_f64.sqrt(), 3. / 14_f64.sqrt())
        );

        assert_eq!(
            v.normalize().magnitude(),
            1.,
            "The magnitude of a normalized vector is 1"
        );
    }

    #[test]
    fn computing_vectors_dot_product() {
        let v1 = Vector::new(1., 2., 3.);
        let v2 = Vector::new(2., 3., 4.);

        assert_eq!(v1.dot(&v2), 20.);
    }

    #[test]
    fn computing_vectors_cross_product() {
        let v1 = Vector::new(1., 2., 3.);
        let v2 = Vector::new(2., 3., 4.);

        assert_eq!(v1.cross(&v2), Vector::new(-1., 2., -1.));
        assert_eq!(v2.cross(&v1), Vector::new(1., -2., 1.));

        let i_hat = Vector::new(1., 0., 0.);
        let j_hat = Vector::new(0., 1., 0.);
        let k_hat = Vector::new(0., 0., 1.);

        assert_eq!(i_hat.cross(&j_hat), k_hat);
        assert_eq!(j_hat.cross(&k_hat), i_hat);
        assert_eq!(k_hat.cross(&i_hat), j_hat);
    }
}
