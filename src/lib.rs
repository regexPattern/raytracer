use std::cmp::PartialEq;
use std::ops::{Add, Sub};

#[derive(Debug)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: u8,
}

impl Tuple {
    fn point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1 }
    }

    fn vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0 }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        const EPSILON: f64 = 0.00001;
        let compare_floating_points = |a: f64, b: f64| a.abs() - b.abs() < EPSILON;

        compare_floating_points(self.x, other.x)
            && compare_floating_points(self.y, other.y)
            && compare_floating_points(self.z, other.z)
            && self.w == other.w
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_are_created_with_w_1() {
        let point = Tuple::point(4.0, -4.0, 3.0);
        let expected = Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 1,
        };

        assert_eq!(point, expected);
    }

    #[test]
    fn vectors_are_created_with_w_0() {
        let vector = Tuple::vector(4.0, -4.0, 3.0);
        let expected = Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 0,
        };

        assert_eq!(vector, expected);
    }

    #[test]
    fn adding_two_tupples() {
        let tuple1 = Tuple {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 1,
        };
        let tuple2 = Tuple {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0,
        };
        let expected = Tuple {
            x: 1.0,
            y: 1.0,
            z: 6.0,
            w: 1,
        };

        assert_eq!(tuple1 + tuple2, expected);
    }

    #[test]
    fn subtracting_two_points() {
        let point1 = Tuple::point(3.0, 2.0, 1.0);
        let point2 = Tuple::point(5.0, 6.0, 7.0);
        let expected = Tuple::vector(-2.0, -4.0, -6.0);

        assert_eq!(point1 - point2, expected);
    }

    #[test]
    fn subtracting_vector_from_point() {
        let point = Tuple::point(3.0, 2.0, 1.0);
        let vector = Tuple::vector(5.0, 6.0, 7.0);
        let expected = Tuple::point(-2.0, -4.0, -6.0);

        assert_eq!(point - vector, expected);
    }
}
