use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::utils;

const POINT_W: f64 = 1.0;
const VECTOR_W: f64 = 0.0;

#[derive(Copy, Clone, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        utils::approximately_eq(self.x, other.x)
            && utils::approximately_eq(self.y, other.y)
            && utils::approximately_eq(self.z, other.z)
            && utils::approximately_eq(self.w, other.w)
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
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

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point(pub Tuple);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Tuple {
            x,
            y,
            z,
            w: POINT_W,
        })
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0 - rhs.0)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(Tuple {
            w: POINT_W,
            ..self.0 * rhs
        })
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(Tuple {
            w: POINT_W,
            ..self.0 / rhs
        })
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(Tuple {
            w: POINT_W,
            ..-self.0
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector(pub Tuple);

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Tuple {
            x,
            y,
            z,
            w: VECTOR_W,
        })
    }

    pub fn magnitude(self) -> f64 {
        (self.0.x.powi(2) + self.0.y.powi(2) + self.0.z.powi(2) + self.0.w.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude();
        self / magnitude
    }

    pub fn dot(self, other: Self) -> f64 {
        self.0.x * other.0.x + self.0.y * other.0.y + self.0.z * other.0.z + self.0.w * other.0.w
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.0.y * other.0.z - self.0.z * other.0.y,
            self.0.z * other.0.x - self.0.x * other.0.z,
            self.0.x * other.0.y - self.0.y * other.0.x,
        )
    }

    pub fn reflect(self, normal: Self) -> Self {
        Self(self.0 - normal.0 * 2.0 * self.dot(normal))
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub fn clamp(value: f64) -> u8 {
        match value {
            x if x <= 0.0 => 0,
            x if x >= 1.0 => 255,
            x => (x * 255.0) as u8,
        }
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        utils::approximately_eq(self.red, other.red)
            && utils::approximately_eq(self.green, other.green)
            && utils::approximately_eq(self.blue, other.blue)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.red - rhs.red,
            self.green - rhs.green,
            self.blue - rhs.blue,
        )
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_creates_a_tuple_with_w_1() {
        let p = Point::new(4.0, -4.0, 3.0);

        assert_eq!(p.0.w, 1.0);
    }

    #[test]
    fn vector_creates_a_tuple_with_w_0() {
        let v = Vector::new(4.0, -4.0, 3.0);

        assert_eq!(v.0.w, 0.0);
    }

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
        let p = Point::new(1.0, 2.0, 3.0);
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(
            Tuple {
                x: 1.0,
                y: 1.0,
                z: 6.0,
                w: 1.0
            },
            t1 + t2
        );
        assert_eq!(p + v1, Point::new(2.0, 4.0, 6.0));
        assert_eq!(v1 + v2, Vector::new(2.0, 4.0, 6.0));
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
            x: 2.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        let t4 = Tuple {
            x: 1.0 + 0.000001,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        let t5 = Tuple {
            x: 1.0 + 0.00001,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };

        assert_eq!(t1, t2);
        assert_ne!(t1, t3);
        assert_eq!(t1, t4);
        assert_ne!(t1, t5);
    }

    #[test]
    fn subtracting_points() {
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
    fn subtracting_vectors() {
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
        let p = Point::new(1.0, -2.0, 3.0);
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(
            -t,
            Tuple {
                x: -1.0,
                y: 2.0,
                z: -3.0,
                w: 4.0
            }
        );
        assert_eq!(-p, Point::new(-1.0, 2.0, -3.0));
        assert_eq!(-v, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let t = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let p = Point::new(1.0, -2.0, 3.0);
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(
            t * 3.5,
            Tuple {
                x: 3.5,
                y: -7.0,
                z: 10.5,
                w: -14.0
            }
        );
        assert_eq!(p * 3.5, Point::new(3.5, -7.0, 10.5));
        assert_eq!(v * 3.5, Vector::new(3.5, -7.0, 10.5));
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let t = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let p = Point::new(1.0, -2.0, 3.0);
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(
            t * 0.5,
            Tuple {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0
            }
        );
        assert_eq!(p * 0.5, Point::new(0.5, -1.0, 1.5));
        assert_eq!(v * 0.5, Vector::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let t = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let p = Point::new(1.0, -2.0, 3.0);
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(
            t / 2.0,
            Tuple {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0
            }
        );
        assert_eq!(p / 2.0, Point::new(0.5, -1.0, 1.5));
        assert_eq!(v / 2.0, Vector::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn computing_magnitude_of_vectors() {
        let unit_i = Vector::new(1.0, 0.0, 0.0);
        let unit_j = Vector::new(0.0, 1.0, 0.0);
        let unit_k = Vector::new(0.0, 1.0, 0.0);

        assert_eq!(1.0, unit_i.magnitude());
        assert_eq!(1.0, unit_j.magnitude());
        assert_eq!(1.0, unit_k.magnitude());
    }

    #[test]
    fn normalizing_vectors() {
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
    fn magnitude_of_a_normalized_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let norm = v.normalize();

        assert_eq!(1.0, norm.magnitude());
    }

    #[test]
    fn dot_product_of_tuples() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(20.0, v1.dot(v2));
    }

    #[test]
    fn cross_product_of_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(v1.cross(v2), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(v1), Vector::new(1.0, -2.0, 1.0));
    }

    #[test]
    fn colors_are_red_green_blue_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);

        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn clamp_color() {
        let c1 = 1.0;
        let c2 = 0.0;
        let c3 = 0.5;
        let c4 = -1.0;
        let c5 = 256.0;

        assert_eq!(Color::clamp(c1), 255);
        assert_eq!(Color::clamp(c2), 0);
        assert_eq!(Color::clamp(c3), 127);
        assert_eq!(Color::clamp(c4), 0);
        assert_eq!(Color::clamp(c5), 255);
    }

    #[test]
    fn getting_black_color() {
        assert_eq!(Color::black(), Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn getting_white_color() {
        assert_eq!(Color::white(), Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn reflecting_a_vector_approaching_at_45_degrees() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::new(0.0, 1.0, 0.0);

        let r = v.reflect(n);

        assert_eq!(r, Vector::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflectin_a_vector_off_a_slanted_surface() {
        let v = Vector::new(0.0, -1.0, 0.0);
        let n = Vector::new(2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0);

        let r = v.reflect(n);

        assert_eq!(r, Vector::new(1.0, 0.0, 0.0));
    }
}
