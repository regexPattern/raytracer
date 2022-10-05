use crate::float;

#[derive(Copy, Clone, Debug)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

#[derive(Debug)]
struct Point(Tuple);

#[derive(Debug)]
struct Vector(Tuple);

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl PartialEq<Tuple> for Point {
    fn eq(&self, other: &Tuple) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Tuple> for Vector {
    fn eq(&self, other: &Tuple) -> bool {
        self.0 == *other
    }
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Tuple { x, y, z, w: 1.0 })
    }
}

impl Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Tuple { x, y, z, w: 0.0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tuple_is_a_point(t: Tuple) -> bool {
        t.w == 1.0
    }

    fn tuple_is_a_vector(t: Tuple) -> bool {
        t.w == 0.0
    }

    #[test]
    fn a_tuple_with_w_1_dot_0_is_a_point() {
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert!(tuple_is_a_point(a));
        assert!(!tuple_is_a_vector(a));
    }

    #[test]
    fn a_tuple_with_w_0_dot_0_is_a_vector() {
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert!(tuple_is_a_vector(a));
        assert!(!tuple_is_a_point(a));
    }

    #[test]
    fn point_creates_tuples_with_w_1() {
        let p = Point::new(4.0, -4.0, 3.0);

        assert_eq!(
            p,
            Tuple {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 1.0
            }
        );
    }

    #[test]
    fn vector_creates_tuples_with_w_0() {
        let v = Vector::new(4.0, -4.0, 3.0);

        assert_eq!(
            v,
            Tuple {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 0.0
            }
        );
    }
}
