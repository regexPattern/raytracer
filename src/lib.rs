#![allow(dead_code)]

use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1. }
    }
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0. }
    }

    pub fn magnitude(&self) -> f64 {
        let coords = [self.x, self.y, self.z, self.w];
        let sum = coords.into_iter().fold(0., |a, b| a + b.powf(2.));
        sum.sqrt()
    }

    // TODO: Discover if there is a way to turn a Struct made of f64 values
    // into an interator, without implementing a custom Iterator trait. It
    // mesk sense that I can't be done, since a Struct can potentially have
    // fields of multiple types.
    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Vector::new(self.x, self.y, self.z) / magnitude
    }

    // TODO: Same as with the `normalize()` method.
    pub fn dot(self, other: Self) -> f64 {
        let self_coords = [self.x, self.y, self.z, self.w];
        let other_coords = [other.x, other.y, other.z, self.w];

        let iter = self_coords.into_iter().zip(other_coords);
        iter.fold(0., |a, (b, c)| a + (b * c))
    }

    pub fn cross(self, other: Self) -> Self {
        Vector::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
        )
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, other: Vector) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, factor: f64) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
            w: self.w * factor,
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
            w: self.w * factor,
        }
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, factor: f64) -> Self {
        Self {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
            w: self.w / factor,
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
            w: self.w / factor,
        }
    }
}

#[cfg(test)]
mod tuple {
    use super::*;

    #[test]
    fn point_has_desired_coordinates() {
        let point = Point {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.,
        };

        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 1.);
    }

    #[test]
    fn vector_has_desired_coordinates() {
        let vector = Vector {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.,
        };

        assert_eq!(vector.x, 4.3);
        assert_eq!(vector.y, -4.2);
        assert_eq!(vector.z, 3.1);
        assert_eq!(vector.w, 0.);
    }

    #[test]
    fn point_constructor_creates_point() {
        let point = Point::new(4.3, -4.2, 3.1);

        assert_eq!(
            point,
            Point {
                x: 4.3,
                y: -4.2,
                z: 3.1,
                w: 1.,
            }
        );
        assert_eq!(point, Point::new(4.3, -4.2, 3.1));
    }

    #[test]
    fn vector_constructor_creates_vector() {
        let vector = Vector::new(4.3, -4.2, 3.1);

        assert_eq!(
            vector,
            Vector {
                x: 4.3,
                y: -4.2,
                z: 3.1,
                w: 0.,
            }
        );
        assert_eq!(vector, Vector::new(4.3, -4.2, 3.1));
    }
}

// The `w` field of a tuple helps us determine which operations are allow between the two types of
// tuples. The resulting tuple has to have a valid `w` value, this means either 0 or 1. For
// example, adding two vectors that have `w: 0` results in a new vector that has `w: 0`, adding a
// point and a vector (and viceversa) results in a new point that has `w: 0 + 1` -> `w: 1`. But
// some opeartions are invalid, such as adding two points, which results in a tuple with `w: 2`,
// which is neither a point nor a tuple.
//
// Common tuple operations:
// * Addition
//      Vector + Vector -> Vector
//      Point + Vector  -> Point (commutative)
//
// * Subtraction
//      Point - Point   -> Point
//      Vector - Vector -> Vector
//      Point - Vector  -> Point
//
// * Negation
//      -Point  -> Point
//      -Vector -> Vector
//
// * Multiplication (Scaling)
//      Point * Scalar  ->  Point
//      Vector * Scalar ->  Vector
//      Point / Scalar  ->  Point
//      Vector / Scalar ->  Vector
//
// Vector specific opeartions:
// * Magnitude
//
#[cfg(test)]
mod operations {
    use super::*;

    #[test]
    fn adding_two_vectors() {
        let vector1 = Vector::new(3., -2., 5.);
        let vector2 = Vector::new(-2., 3., 1.);

        assert_eq!(vector1 + vector2, Vector::new(1., 1., 6.));
    }

    #[test]
    fn adding_vector_to_point() {
        let point = Point::new(3., -2., 5.);
        let vector = Vector::new(-2., 3., 1.);

        assert_eq!(point + vector, Point::new(1., 1., 6.));
    }

    #[test]
    fn adding_point_to_vector() {
        let point = Point::new(3., -2., 5.);
        let vector = Vector::new(-2., 3., 1.);

        assert_eq!(vector + point, Point::new(1., 1., 6.));
    }

    #[test]
    fn subtracting_two_points() {
        let point1 = Point::new(3., 2., 1.);
        let point2 = Point::new(5., 6., 7.);

        assert_eq!(point1 - point2, Vector::new(-2., -4., -6.));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let point = Point::new(3., 2., 1.);
        let vector = Vector::new(5., 6., 7.);

        assert_eq!(point - vector, Point::new(-2., -4., -6.));
    }

    #[test]
    fn subtracting_two_vectors() {
        let vector1 = Vector::new(3., 2., 1.);
        let vector2 = Vector::new(5., 6., 7.);

        assert_eq!(vector1 - vector2, Vector::new(-2., -4., -6.));
    }

    #[test]
    fn subtracting_vector_from_the_zero_vector() {
        let zero = Vector::new(0., 0., 0.);
        let vector = Vector::new(1., -2., 3.);

        assert_eq!(zero - vector, Vector::new(-1., 2., -3.));
    }

    #[test]
    fn negating_point() {
        let point = Point {
            x: 1.,
            y: -2.,
            z: 3.,
            w: -4.,
        };

        assert_eq!(
            -point,
            Point {
                x: -1.,
                y: 2.,
                z: -3.,
                w: 4.,
            }
        );
    }

    #[test]
    fn negating_vector() {
        let vector = Vector {
            x: 1.,
            y: -2.,
            z: 3.,
            w: -4.,
        };

        assert_eq!(
            -vector,
            Vector {
                x: -1.,
                y: 2.,
                z: -3.,
                w: 4.,
            }
        );
    }

    #[test]
    fn scaling_point() {
        let point = Point {
            x: 1.,
            y: -2.,
            z: 3.,
            w: -4.,
        };

        let point = point * 3.5;
        assert_eq!(
            point,
            Point {
                x: 3.5,
                y: -7.,
                z: 10.5,
                w: -14.,
            }
        );

        let point = point / 3.5;
        assert_eq!(
            point,
            Point {
                x: 1.,
                y: -2.,
                z: 3.,
                w: -4.,
            }
        );
    }

    #[test]
    fn scaling_vector() {
        let vector = Vector {
            x: 1.,
            y: -2.,
            z: 3.,
            w: -4.,
        };

        let vector = vector * 3.5;
        assert_eq!(
            vector,
            Vector {
                x: 3.5,
                y: -7.,
                z: 10.5,
                w: -14.,
            }
        );

        let vector = vector / 3.5;
        assert_eq!(
            vector,
            Vector {
                x: 1.,
                y: -2.,
                z: 3.,
                w: -4.,
            }
        );
    }

    #[test]
    fn computing_magnitude_of_vector() {
        let zero = Vector::new(0., 0., 0.);
        assert_eq!(zero.magnitude(), 0.);

        let vector = Vector::new(1., 0., 0.);
        assert_eq!(vector.magnitude(), 1.);

        let vector = Vector::new(0., 1., 0.);
        assert_eq!(vector.magnitude(), 1.);

        let vector = Vector::new(0., 0., 1.);
        assert_eq!(vector.magnitude(), 1.);

        let vector = Vector::new(1., 2., 3.);
        assert_eq!(vector.magnitude(), (14. as f64).sqrt());

        let vector = Vector::new(-1., -2., -3.);
        assert_eq!(vector.magnitude(), (14. as f64).sqrt());
    }

    #[test]
    fn normalizing_vector_returns_unit_vector() {
        let vector = Vector::new(4., 0., 0.);
        let norm = vector.normalize();

        assert_eq!(norm.magnitude(), 1.);

        let vector = Vector::new(1., 2., 3.);

        let norm = vector.normalize();
        let magnitude = vector.magnitude();

        assert_eq!(
            norm,
            Vector {
                x: 1. / magnitude,
                y: 2. / magnitude,
                z: 3. / magnitude,
                w: 0.
            }
        );

        let norm = vector.normalize();

        assert_eq!(norm.magnitude(), 1.);
    }

    #[test]
    fn computing_dot_product_of_two_vectors() {
        let vector1 = Vector::new(1., 2., 3.);
        let vector2 = Vector::new(2., 3., 4.);

        assert_eq!(vector1.dot(vector2), 20.);
    }

    #[test]
    fn computing_cross_product_of_two_vectors() {
        let vector1 = Vector::new(1., 2., 3.);
        let vector2 = Vector::new(2., 3., 4.);

        assert_eq!(vector1.cross(vector2), Vector::new(-1., 2., -1.));
        assert_eq!(vector2.cross(vector1), Vector::new(1., -2., 1.));

        let norm_x = Vector::new(1., 0., 0.);
        let norm_y = Vector::new(0., 1., 0.);
        let norm_z = Vector::new(0., 0., 1.);

        assert_eq!(norm_x.cross(norm_y), norm_z);
    }
}
