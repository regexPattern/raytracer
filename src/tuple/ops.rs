use std::ops::{Add, Div, Mul, Neg, Sub};

use super::{Point, Vector, Tuple};

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
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
        Point(self.0 + rhs.0)
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
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

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
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

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        rhs * self
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector(self * rhs.0)
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_two_tuples() {
        let t1 = Tuple {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 1.0,
        };
        let t2 = Tuple {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0,
        };

        assert_eq!(
            t1 + t2,
            Tuple {
                x: 1.0,
                y: 1.0,
                z: 6.0,
                w: 1.0
            }
        );
    }

    #[test]
    fn adding_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);

        assert_eq!(v1 + v2, Vector::new(5.0, 7.0, 9.0));
        assert_eq!(v1 + v2, v2 + v1, "`Vector` addition is commutative");
    }

    #[test]
    fn adding_point_and_vector() {
        let p = Point::new(1.0, 2.0, 3.0);
        let v = Vector::new(4.0, 5.0, 6.0);

        assert_eq!(p + v, Point::new(5.0, 7.0, 9.0));
        assert_eq!(p + v, v + p, "`Point` and `Vector` addition is commutative");
    }

    #[test]
    fn subtracting_two_tuples() {
        let t1 = Tuple {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 1.0,
        };
        let t2 = Tuple {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0,
        };

        assert_eq!(
            t1 - t2,
            Tuple {
                x: 5.0,
                y: -5.0,
                z: 4.0,
                w: 1.0
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
    fn subtracting_a_vector_from_point() {
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
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = Vector::new(0.0, 0.0, 0.0);
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(zero - v, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negating_a_tuple() {
        let t = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        assert_eq!(
            -t,
            Tuple {
                x: -1.0,
                y: 2.0,
                z: -3.0,
                w: 4.0
            }
        );
    }

    #[test]
    fn negating_a_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(-v, Vector::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let t = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        assert_eq!(
            t * 3.5,
            Tuple {
                x: 3.5,
                y: -7.0,
                z: 10.5,
                w: -14.0
            }
        );
        assert_eq!(
            t * 3.5,
            3.5 * t,
            "`f64` and `Tuple` multiplication is commutative"
        );
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let t = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        assert_eq!(
            t * 0.5,
            Tuple {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0
            }
        );
    }

    #[test]
    fn multiplying_a_vector_by_a_scalar() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v * 2.0, Vector::new(2.0, 4.0, 6.0));
        assert_eq!(
            v * 2.0,
            2.0 * v,
            "`f64` and `Vector` multiplication is commutative"
        );
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let t = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        assert_eq!(
            t / 2.0,
            Tuple {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0
            }
        );
    }

    #[test]
    fn dividing_a_vector_by_a_scalar() {
        let v = Vector::new(2.0, 4.0, 6.0);

        assert_eq!(v / 2.0, Vector::new(1.0, 2.0, 3.0));
    }
}
