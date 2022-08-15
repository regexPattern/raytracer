use std::ops::{Add, Div, Mul, Neg, Sub};

// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
#[derive(Copy, Clone, Debug)]
struct Scalar(f64);

impl PartialEq for Scalar {
    fn eq(&self, other: &Scalar) -> bool {
        (self.0 - other.0).abs() < f64::EPSILON
    }
}

impl Add for Scalar {
    type Output = Scalar;

    fn add(self, rhs: Scalar) -> Scalar {
        Scalar(self.0 + rhs.0)
    }
}

impl Sub for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: Scalar) -> Scalar {
        Scalar(self.0 - rhs.0)
    }
}

impl Mul for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: Self) -> Scalar {
        Scalar(self.0 * rhs.0)
    }
}

#[derive(Copy, Clone, Debug)]
struct Tuple {
    x: Scalar,
    y: Scalar,
    z: Scalar,
}

impl Tuple {
    fn new(x: f64, y: f64, z: f64) -> Tuple {
        let x = Scalar(x);
        let y = Scalar(y);
        let z = Scalar(z);

        Tuple { x, y, z }
    }

    // TODO: Me gustaria usar el trait `Into` aca.
    fn coordinates(&self) -> (f64, f64, f64) {
        (self.x.0, self.y.0, self.z.0)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Tuple {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;

        Tuple::new(x.0, y.0, z.0)
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Tuple {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;

        Tuple::new(x.0, y.0, z.0)
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple::new(0.0, 0.0, 0.0) - self
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Tuple {
        let (x, y, z) = self.coordinates();

        Tuple::new(x * rhs, y * rhs, z * rhs)
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Tuple {
        let (x, y, z) = self.coordinates();

        Tuple::new(x / rhs, y / rhs, z / rhs)
    }
}

#[derive(Copy, Clone, Debug)]
struct Point {
    tuple: Tuple,
    w: Scalar,
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            tuple: Tuple::new(x, y, z),
            w: Scalar(1.0),
        }
    }
}

impl From<Tuple> for Point {
    fn from(tuple: Tuple) -> Point {
        let (x, y, z) = tuple.coordinates();

        Point::new(x, y, z)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.tuple == other.tuple && self.w == other.w
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Point {
        Point::from(self.tuple + rhs.tuple)
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Vector {
        Vector::from(self.tuple - rhs.tuple)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Point {
        Point::from(self.tuple - rhs.tuple)
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point::from(-self.tuple)
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Point {
        Point::from(self.tuple * rhs)
    }
}

#[derive(Copy, Clone, Debug)]
struct Vector {
    tuple: Tuple,
    w: Scalar,
}

impl Vector {
    fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector {
            tuple: Tuple::new(x, y, z),
            w: Scalar(0.0),
        }
    }

    fn magnitude(&self) -> Scalar {
        // TODO: Ver si aqui puedo hacer `Into` con `f64` para Coordinate.
        let (x, y, z) = self.tuple.coordinates();
        let coordinates = [x, y, z];
        let magnitude = coordinates
            .iter()
            .fold(0.0, |sum, n| sum + n.powi(2))
            .sqrt();

        Scalar(magnitude)
    }

    fn normalize(self) -> Vector {
        let magnitude = self.magnitude();
        match magnitude {
            x if x == Scalar(0.0) => Vector::new(0.0, 0.0, 0.0),
            _ => Vector::from(self.tuple / self.magnitude().0),
        }
    }

    fn dot(self, rhs: Vector) -> Scalar {
        // TODO: Debe haber una mejor forma de hacer esto.
        let (x, y, z) = self.tuple.coordinates();
        let self_coordinates = [x, y, z];

        let (x, y, z) = rhs.tuple.coordinates();
        let rhs_coordinates = [x, y, z];

        let product = self_coordinates
            .iter()
            .zip(rhs_coordinates)
            .fold(0.0, |sum, (a, b)| sum + (a * b));

        Scalar(product)
    }

    fn cross(self, rhs: Vector) -> Vector {
        let x = self.tuple.y * rhs.tuple.z - self.tuple.z * rhs.tuple.y;
        let y = self.tuple.z * rhs.tuple.x - self.tuple.x * rhs.tuple.z;
        let z = self.tuple.x * rhs.tuple.y - self.tuple.y * rhs.tuple.x;

        Vector::from(Tuple::new(x.0, y.0, z.0))
    }
}

impl From<Tuple> for Vector {
    fn from(tuple: Tuple) -> Vector {
        let (x, y, z) = tuple.coordinates();

        Vector::new(x, y, z)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.tuple == other.tuple && self.w == other.w
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector::from(self.tuple + rhs.tuple)
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point::from(self.tuple + rhs.tuple)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector::from(self.tuple - rhs.tuple)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector::from(-self.tuple)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        Vector::from(self.tuple * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparing_two_scalar() {
        let c1 = Scalar(1.0);
        let c2 = Scalar(1.0);
        let c3 = Scalar(1.0 + f64::EPSILON);

        assert_eq!(c1, c2);
        assert_ne!(c1, c3);
    }

    #[test]
    fn comparing_two_tuples() {
        let t1 = Tuple::new(1.0, 2.0, 3.0);
        let t2 = Tuple::new(1.0, 2.0, 3.0);
        let t3 = Tuple::new(2.0, 3.0, 4.0);

        assert_eq!(t1, t2);
        assert_ne!(t1, t3);
    }

    #[test]
    fn getting_tuple_coordinates() {
        let t = Tuple::new(1.0, 2.0, 3.0);

        assert_eq!(t.coordinates(), (1.0, 2.0, 3.0));
    }

    #[test]
    fn creating_point() {
        let p = Point::new(1.0, 2.0, 3.0);

        assert_eq!(p.tuple, Tuple::new(1.0, 2.0, 3.0));
        assert_eq!(p.w, Scalar(1.0));
    }

    #[test]
    fn creating_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v.tuple, Tuple::new(1.0, 2.0, 3.0));
        assert_eq!(v.w, Scalar(0.0));
    }

    #[test]
    fn creating_point_from_tuple() {
        let p = Point::from(Tuple::new(1.0, 2.0, 3.0));

        assert_eq!(p, Point::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn creating_vector_from_tuple() {
        let v = Vector::from(Tuple::new(1.0, 2.0, 3.0));

        assert_eq!(v, Vector::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn adding_two_coordiantes() {
        let c1 = Scalar(1.0);
        let c2 = Scalar(2.0);

        assert_eq!(c1 + c2, Scalar(3.0));
    }

    #[test]
    fn adding_two_tuples() {
        let t1 = Tuple::new(1.0, 2.0, 3.0);
        let t2 = Tuple::new(2.0, 3.0, 4.0);

        assert_eq!(t1 + t2, Tuple::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn adding_vector_to_vector() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(v1 + v2, Vector::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn adding_vector_to_point() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(p + v, Point::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn adding_point_to_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(v + p, Point::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(v1 - v2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(p - v, Point::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negating_tuple() {
        let t = Tuple::new(1.0, 2.0, 3.0);

        assert_eq!(-t, Tuple::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn negating_point() {
        let p = Point::new(1.0, 2.0, 3.0);

        assert_eq!(-p, Point::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn negating_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(-v, Vector::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn scaling_tuple() {
        let t = Tuple::new(1.0, 2.0, 3.0);

        assert_eq!(t * 2.0, Tuple::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn scaling_point() {
        let p = Point::new(1.0, 2.0, 3.0);

        assert_eq!(p * 2.0, Point::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn scaling_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v * 2.0, Vector::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector_magnitude() {
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 1.0, 0.0);
        let v3 = Vector::new(0.0, 0.0, 1.0);
        let v4 = Vector::new(1.0, 2.0, 3.0);
        let v5 = Vector::new(0.0, 0.0, 0.0);

        assert_eq!(v1.magnitude(), Scalar(1.0));
        assert_eq!(v2.magnitude(), Scalar(1.0));
        assert_eq!(v3.magnitude(), Scalar(1.0));
        assert_eq!(v4.magnitude(), Scalar(14_f64.sqrt()));
        assert_eq!(v5.magnitude(), Scalar(0.0));
    }

    #[test]
    fn vector_normalization() {
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
    fn normalize_null_vector() {
        let v = Vector::new(0.0, 0.0, 0.0);

        assert_eq!(v.normalize(), Vector::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn normalized_vector_magnitude() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v.normalize().magnitude(), Scalar(1.0));
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(v1.dot(v2), Scalar(20.0));
        assert_eq!(v2.dot(v1), Scalar(20.0));
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(v1.cross(v2), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(v1), Vector::new(1.0, -2.0, 1.0));
    }
}
