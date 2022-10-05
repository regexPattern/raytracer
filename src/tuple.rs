mod ops;

use crate::float;

#[derive(Copy, Clone, Debug)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point(Tuple);

#[derive(Copy, Clone, Debug, PartialEq)]
struct Vector(Tuple);

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        float::approx(self.x, other.x)
            && float::approx(self.y, other.y)
            && float::approx(self.z, other.z)
            && float::approx(self.w, other.w)
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

    fn magnitude(&self) -> f64 {
        let Tuple { x, y, z, .. } = self.0;
        (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
    }

    fn normalize(self) -> Self {
        self / self.magnitude()
    }

    fn dot(&self, rhs: &Self) -> f64 {
        let Tuple {
            x: x1,
            y: y1,
            z: z1,
            ..
        } = self.0;
        let Tuple {
            x: x2,
            y: y2,
            z: z2,
            ..
        } = rhs.0;

        x1 * x2 + y1 * y2 + z1 * z2
    }

    fn cross(&self, rhs: &Self) -> Self {
        let Tuple {
            x: x1,
            y: y1,
            z: z1,
            ..
        } = self.0;
        let Tuple {
            x: x2,
            y: y2,
            z: z2,
            ..
        } = rhs.0;

        Self::new(y1 * z2 - z1 * y2, z1 * x2 - x1 * z2, x1 * y2 - y1 * x2)
    }
}

#[cfg(test)]
mod tests {
    use crate::float::assert_approx;

    use super::*;

    fn tuple_is_a_point(t: Tuple) -> bool {
        float::approx(t.w, 1.0)
    }

    fn tuple_is_a_vector(t: Tuple) -> bool {
        float::approx(t.w, 0.0)
    }

    #[test]
    fn a_tuple_with_w_1_dot_0_is_a_point() {
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };

        assert_approx(a.x, 4.3);
        assert_approx(a.y, -4.2);
        assert_approx(a.z, 3.1);
        assert_approx(a.w, 1.0);
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

        assert_approx(a.x, 4.3);
        assert_approx(a.y, -4.2);
        assert_approx(a.z, 3.1);
        assert_approx(a.w, 0.0);
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

    #[test]
    fn computing_the_magnitude_of_the_unit_vectors() {
        let i = Vector::new(1.0, 0.0, 0.0);
        let j = Vector::new(0.0, 1.0, 0.0);
        let k = Vector::new(0.0, 0.0, 1.0);

        /* TODO: Convert `assert_approx` to a macro that takes a string.
        assert_approx!(i.magnitude(), 1.0, "Magnitude of î is `1.0`");
        assert_approx!(j.magnitude(), 1.0, "Magnitude of ĵ is `1.0`");
        assert_approx!(k.magnitude(), 1.0, "Magnitude of k̂ is `1.0`"); */

        assert_approx(i.magnitude(), 1.0);
        assert_approx(j.magnitude(), 1.0);
        assert_approx(k.magnitude(), 1.0);
    }

    #[test]
    fn computing_the_magnitude_of_non_unit_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(-1.0, -2.0, -3.0);

        assert_approx(v1.magnitude(), 14_f64.sqrt());
        assert_approx(v2.magnitude(), 14_f64.sqrt());
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
    fn the_dot_product_of_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_approx(v1.dot(&v2), 20.0);
        // TODO: Refactor this when I implement this macro.
        // assert_approx!(v2.dot(&v1), 20.0, "`Vector` dot product is commutative");
        assert_approx(v2.dot(&v1), 20.0);
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(v1.cross(&v2), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(&v1), -v1.cross(&v2));
    }

    #[test]
    fn the_cross_product_of_the_unit_vectors() {
        let i = Vector::new(1.0, 0.0, 0.0);
        let j = Vector::new(0.0, 1.0, 0.0);
        let k = Vector::new(0.0, 0.0, 1.0);

        assert_eq!(i.cross(&j), k);
        assert_eq!(i.cross(&k), -j);
        assert_eq!(j.cross(&i), -k);
        assert_eq!(j.cross(&k), i);
        assert_eq!(k.cross(&i), j);
        assert_eq!(k.cross(&j), -i);
    }
}
