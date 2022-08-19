use crate::utils;
use std::ops::{Add, Div, Mul, Neg, Sub};

const POINT_W: f64 = 1.0;
const VECTOR_W: f64 = 0.0;

#[derive(Copy, Clone, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::new(x, y, z, POINT_W)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::new(x, y, z, VECTOR_W)
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Tuple {
        let magnitude = self.magnitude();
        self / magnitude
    }

    pub fn dot(&self, other: &Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Tuple) -> Tuple {
        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn is_point(t: &Tuple) -> bool {
        utils::approximately_eq(t.w, POINT_W)
    }

    pub fn is_vector(t: &Tuple) -> bool {
        utils::approximately_eq(t.w, VECTOR_W)
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Self::Output {
        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Add<&Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: &Tuple) -> Self::Output {
        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        utils::approximately_eq(self.x, other.x)
            && utils::approximately_eq(self.y, other.y)
            && utils::approximately_eq(self.z, other.z)
            && utils::approximately_eq(self.w, other.w)
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Self::Output {
        Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tuple_with_a_w_1_dot_0_is_a_point() {
        let t = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert!(Tuple::is_point(&t));
    }

    #[test]
    fn a_tuple_with_a_w_0_dot_0_is_a_vector() {
        let t = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert!(Tuple::is_vector(&t));
    }

    #[test]
    fn point_creates_a_tuple_with_w_1_dot_0() {
        let t = Tuple::point(4.0, -4.0, 3.0);

        assert!(Tuple::is_point(&t));
    }

    #[test]
    fn vector_creates_a_tuple_with_w_1_dot_0() {
        let t = Tuple::vector(4.0, -4.0, 3.0);

        assert!(Tuple::is_vector(&t));
    }

    #[test]
    fn adding_two_tuples() {
        let t1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let t2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        assert_eq!(Tuple::new(1.0, 1.0, 6.0, 1.0), t1 + t2);
    }

    #[test]
    fn comparing_tuples() {
        let t1 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let t2 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let t3 = Tuple::new(2.0, 2.0, 3.0, 4.0);
        let t4 = Tuple::new(1.0 + f64::EPSILON, 2.0, 3.0, 4.0);
        let t5 = Tuple::new(1.0 + (2.0 * f64::EPSILON), 2.0, 3.0, 4.0);

        assert_eq!(t1, t2);
        assert_ne!(t1, t3);
        assert_eq!(t1, t4);
        assert_ne!(t1, t5);
    }

    #[test]
    fn subtracting_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(p - v, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(v1 - v2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);

        assert_eq!(zero - v, Tuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negating_a_tuple() {
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(-t, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(t * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(t * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(t / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn computing_magnitude_of_vectors() {
        let unit_i = Tuple::vector(1.0, 0.0, 0.0);
        let unit_j = Tuple::vector(0.0, 1.0, 0.0);
        let unit_k = Tuple::vector(0.0, 1.0, 0.0);

        assert_eq!(1.0, unit_i.magnitude());
        assert_eq!(1.0, unit_j.magnitude());
        assert_eq!(1.0, unit_k.magnitude());
    }

    #[test]
    fn normalizing_vectors() {
        let v1 = Tuple::vector(4.0, 0.0, 0.0);
        let v2 = Tuple::vector(1.0, 2.0, 3.0);

        assert_eq!(v1.normalize(), Tuple::vector(1.0, 0.0, 0.0));
        assert_eq!(
            v2.normalize(),
            Tuple::vector(
                1.0 / 14_f64.sqrt(),
                2.0 / 14_f64.sqrt(),
                3.0 / 14_f64.sqrt()
            )
        );
    }

    #[test]
    fn magnitude_of_a_normalized_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        let norm = v.normalize();

        assert_eq!(1.0, norm.magnitude());
    }

    #[test]
    fn dot_product_of_tuples() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(20.0, v1.dot(&v2));
    }

    #[test]
    fn cross_product_of_vectors() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(v1.cross(&v2), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(&v1), Tuple::vector(1.0, -2.0, 1.0));
    }
}
