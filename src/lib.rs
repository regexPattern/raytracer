use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug)]
struct Tuple(f64, f64, f64);

impl Tuple {
    fn new(a: f64, b: f64, c: f64) -> Self {
        Tuple(a, b, c)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        let diff = self - other;
        let diff = [diff.0, diff.1, diff.2];
        !diff.iter().any(|&i| i.abs() > f64::EPSILON)
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Self) -> Self::Output {
        Tuple::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for &Tuple {
    type Output = Tuple;

    fn sub(self, other: Self) -> Self::Output {
        Tuple::new(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

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
    type Output = Coordinate;

    fn add(self, other: Self) -> Self::Output {
        Coordinate {
            tuple: self.tuple + other.tuple,
            w: self.w + other.w,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Point(Coordinate);

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point(Coordinate::new(x, y, z, 1.))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, other: Vector) -> Self::Output {
        Point(self.0 + other.0)
    }
}

#[derive(Copy, Clone, Debug)]
struct Vector(Coordinate);

impl Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector(Coordinate::new(x, y, z, 0.))
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Add for Vector {
    type Output = Vector;

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

#[cfg(test)]
mod create {

    use super::*;

    #[test]
    fn create_tuple() {
        let t = Tuple::new(4.3, -4.2, 3.1);

        assert_eq!(t.0, 4.3);
        assert_eq!(t.1, -4.2);
        assert_eq!(t.2, 3.1);
    }

    #[test]
    fn tuples_are_equal() {
        let t1 = Tuple::new(1., 2., 3.);
        let t2 = Tuple::new(1., 2., 3.);

        assert_eq!(t1, t2);

        let t1 = Tuple::new(1., 2., 3.);
        let t2 = Tuple::new(4., 5., 6.);

        assert_ne!(t1, t2);

        let t1 = Tuple::new(1., 2., 3.);
        let t2 = Tuple::new(1. + f64::EPSILON, 2., 3.);
        let t3 = Tuple::new(1. + (f64::EPSILON * 2.), 2., 3.);

        assert_eq!(t1, t2);
        assert_ne!(t1, t3);
    }

    #[test]
    fn create_coordiante() {
        let c = Coordinate::new(4.3, -4.2, 3.1, 1.);
        let t = Tuple::new(4.3, -4.2, 3.1);

        assert_eq!(c.tuple, t);
        assert_eq!(c.w, 1.);
    }

    #[test]
    fn coordinates_are_equal() {
        let c1 = Coordinate::new(1., 2., 3., 1.);
        let c2 = Coordinate::new(1., 2., 3., 1.);

        assert_eq!(c1, c2);
    }

    #[test]
    fn create_point() {
        let p = Point::new(4.3, -4.2, 3.1);
        let c = Coordinate::new(4.3, -4.2, 3.1, 1.);

        assert_eq!(p.0, c, "point is a coordinate with w 1");
    }

    #[test]
    fn create_vector() {
        let v = Vector::new(4.3, -4.2, 3.1);
        let c = Coordinate::new(4.3, -4.2, 3.1, 0.);

        assert_eq!(v.0, c, "vector is a coordinate with w 0");
    }
}

#[cfg(test)]
mod ops {
    use super::*;

    #[test]
    fn adding_tuples() {
        let t1 = Tuple::new(3., -2., 5.);
        let t2 = Tuple::new(-2., 3., 1.);

        assert_eq!(t1 + t2, Tuple::new(1., 1., 6.));
    }

    #[test]
    fn adding_coordinates() {
        let c1 = Coordinate::new(3., -2., 5., 1.);
        let c2 = Coordinate::new(-2., 3., 1., 0.);

        assert_eq!(c1 + c2, Coordinate::new(1., 1., 6., 1.));
    }

    #[test]
    fn adding_vector_to_point() {
        let p = Point::new(3., -2., 5.);
        let v = Vector::new(-2., 3., 1.);

        assert_eq!(p + v, Point::new(1., 1., 6.));
    }

    #[test]
    fn adding_point_to_vector() {
        let p = Point::new(3., -2., 5.);
        let v = Vector::new(-2., 3., 1.);

        assert_eq!(v + p, Point::new(1., 1., 6.));
    }

    #[test]
    fn adding_vector_to_vector() {
        let v1 = Vector::new(3., -2., 5.);
        let v2 = Vector::new(-2., 3., 1.);

        assert_eq!(v1 + v2, Vector::new(1., 1., 6.));
    }
}
