use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::utils;

const POINT_W: f64 = 1.0;
const VECTOR_W: f64 = 0.0;

#[derive(Debug, PartialEq, Eq)]
pub struct NormalizeNullVectorError;

#[derive(Debug, PartialEq, Eq)]
pub struct DivisionByZeroError;

#[derive(Copy, Clone, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point(pub Tuple);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector(pub Tuple);

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        utils::approx(self.x, other.x)
            && utils::approx(self.y, other.y)
            && utils::approx(self.z, other.z)
            && utils::approx(self.w, other.w)
    }
}

impl Point {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        let w = POINT_W;

        Self(Tuple { x, y, z, w })
    }
}

impl Vector {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        let w = VECTOR_W;

        Self(Tuple { x, y, z, w })
    }

    pub fn magnitude(self) -> f64 {
        (self.0.x.powi(2) + self.0.y.powi(2) + self.0.z.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Result<Self, NormalizeNullVectorError> {
        (self / self.magnitude()).map_err(|_| NormalizeNullVectorError)
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.0.x * rhs.0.x + self.0.y * rhs.0.y + self.0.z * rhs.0.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        let x = self.0.y * rhs.0.z - self.0.z * rhs.0.y;
        let y = self.0.z * rhs.0.x - self.0.x * rhs.0.z;
        let z = self.0.x * rhs.0.y - self.0.y * rhs.0.x;

        Self::new(x, y, z)
    }

    pub fn reflect(self, normal: Self) -> Self {
        self - normal * 2.0 * self.dot(normal)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        let w = self.w + rhs.w;

        Self { x, y, z, w }
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        rhs + self
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        let w = self.w - rhs.w;

        Self { x, y, z, w }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0 - rhs.0)
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(0.0, 0.0, 0.0) - self
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let x = self.0.x * rhs;
        let y = self.0.y * rhs;
        let z = self.0.z * rhs;

        Self::new(x, y, z)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vector {
    type Output = Result<Self, DivisionByZeroError>;

    fn div(self, rhs: f64) -> Self::Output {
        (!utils::approx(rhs, 0.0))
            .then_some(self * (1.0 / rhs))
            .ok_or(DivisionByZeroError)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;

    use super::*;

    fn is_a_point(t: Tuple) -> bool {
        utils::approx(t.w, 1.0)
    }

    fn is_a_vector(t: Tuple) -> bool {
        utils::approx(t.w, 0.0)
    }

    #[test]
    fn a_tuple_with_w_1_0_is_a_point() {
        let p = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };

        assert_approx!(p.x, 4.3);
        assert_approx!(p.y, -4.2);
        assert_approx!(p.z, 3.1);
        assert_approx!(p.w, 1.0);

        assert!(is_a_point(p));
        assert!(!is_a_vector(p));
    }

    #[test]
    fn a_tuple_with_w_0_0_is_a_vector() {
        let v = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };

        assert_approx!(v.x, 4.3);
        assert_approx!(v.y, -4.2);
        assert_approx!(v.z, 3.1);
        assert_approx!(v.w, 0.0);

        assert!(is_a_vector(v));
        assert!(!is_a_point(v));
    }

    #[test]
    fn comparing_tuples() {
        let t1 = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };

        let t2 = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };

        let t3 = Tuple {
            x: 4.0,
            y: 3.0,
            z: 2.0,
            w: 1.0,
        };

        assert_eq!(t1, t2);
        assert_ne!(t1, t3);
    }

    #[test]
    fn point_new_creates_tuples_with_w_1_0() {
        let p = Point::new(4.0, -4.0, 3.0);

        assert_eq!(
            p.0,
            Tuple {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 1.0,
            }
        );
    }

    #[test]
    fn vector_new_creates_tuples_with_w_0_0() {
        let v = Vector::new(4.0, -4.0, 3.0);

        assert_eq!(
            v.0,
            Tuple {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 0.0,
            }
        );
    }

    #[test]
    fn adding_two_tuples() {
        let t1 = Tuple {
            x: -3.0,
            y: -2.0,
            z: 5.0,
            w: 7.0,
        };

        let t2 = Tuple {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: -4.0,
        };

        assert_eq!(
            t1 + t2,
            Tuple {
                x: -5.0,
                y: 1.0,
                z: 6.0,
                w: 3.0,
            }
        );
        assert_eq!(t1 + t2, t2 + t1);
    }

    #[test]
    fn adding_a_point_and_a_vector() {
        let p = Point::new(-3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);

        assert_eq!(p + v, Point::new(-5.0, 1.0, 6.0));
        assert_eq!(p + v, v + p);
    }

    #[test]
    fn adding_two_vectors() {
        let v1 = Vector::new(-3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);

        assert_eq!(v1 + v2, Vector::new(-5.0, 1.0, 6.0));
        assert_eq!(v1 + v2, v2 + v1);
    }

    #[test]
    fn subtracting_two_tuples() {
        let t1 = Tuple {
            x: -3.0,
            y: -2.0,
            z: 5.0,
            w: 7.0,
        };

        let t2 = Tuple {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: -4.0,
        };

        assert_eq!(
            t1 - t2,
            Tuple {
                x: -1.0,
                y: -5.0,
                z: 4.0,
                w: 11.0,
            }
        );
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(p - v, Point::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(v1 - v2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_a_vector_from_the_null_vector() {
        let null = Vector::new(0.0, 0.0, 0.0);
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(null - v, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negating_a_vector() {
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(-v, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn multiplying_a_vector_by_a_scalar() {
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(v * 3.5, Vector::new(3.5, -7.0, 10.5));
        assert_eq!(v * 3.5, 3.5 * v);
        assert_eq!(v * 0.5, Vector::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn dividing_a_vector_by_a_non_zero_scalar() {
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(v / 2.0, Ok(Vector::new(0.5, -1.0, 1.5)));
    }

    #[test]
    fn trying_to_divide_a_vector_by_zero() {
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(v / 0.0, Err(DivisionByZeroError));
    }

    #[test]
    fn computing_the_magnitude_of_unit_vectors() {
        let i_hat = Vector::new(1.0, 0.0, 0.0);
        let j_hat = Vector::new(0.0, 1.0, 0.0);
        let k_hat = Vector::new(0.0, 0.0, 1.0);

        assert_approx!(i_hat.magnitude(), 1.0);
        assert_approx!(j_hat.magnitude(), 1.0);
        assert_approx!(k_hat.magnitude(), 1.0);
    }

    #[test]
    fn computing_the_magnitude_of_an_arbitrary_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_approx!(v.magnitude(), 14_f64.sqrt());
        assert_approx!((-v).magnitude(), 14_f64.sqrt());
    }

    #[test]
    fn normalizing_non_null_vectors() {
        let v1 = Vector::new(4.0, 0.0, 0.0);
        let v2 = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v1.normalize(), Ok(Vector::new(1.0, 0.0, 0.0)));
        assert_eq!(
            v2.normalize(),
            Ok(Vector::new(
                1.0 / 14_f64.sqrt(),
                2.0 / 14_f64.sqrt(),
                3.0 / 14_f64.sqrt()
            ))
        );
        assert_approx!(v2.normalize().unwrap().magnitude(), 1.0);
    }

    #[test]
    fn trying_to_normalize_a_null_vector() {
        let null = Vector::new(0.0, 0.0, 0.0);

        assert_eq!(null.normalize(), Err(NormalizeNullVectorError));
    }

    #[test]
    fn the_dot_product_of_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_approx!(v1.dot(v2), 20.0);
        assert_approx!(v1.dot(v2), v2.dot(v1));
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let null = Vector::new(0.0, 0.0, 0.0);

        assert_eq!(v1.cross(v2), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(v1), -v1.cross(v2));
        assert_eq!(v1.cross(v1), null);
        assert_eq!(v1.cross(null), null);
    }

    #[test]
    fn reflecting_a_vector_approaching_at_45_degrees() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::new(0.0, 1.0, 0.0);

        let r = v.reflect(n);

        assert_eq!(r, Vector::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = Vector::new(0.0, -1.0, 0.0);
        let n = Vector::new(2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0);

        let r = v.reflect(n);

        assert_eq!(r, Vector::new(1.0, 0.0, 0.0));
    }
}
