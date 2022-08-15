use std::ops::{Add, Sub};

// TODO: Ver si puedo unificar la creacion tanto de puntos como de vectores para que queden
// unificadas en el `impl` block de cada tipo. Ahora por ejemplo, cuando implemento diferentes
// traits como `Add` estoy utilizando Struct { ... } para crear estos tipos en vez del constructor
// `Struct::new()`. Poria implementar el trait `From` para ver si puedo hacer constructores que
// directamente reciban tuples.

const POINT_COORDINATE: Coordinate = Coordinate(1.0);
const VECTOR_COORDINATE: Coordinate = Coordinate(0.0);

#[derive(Debug)]
struct Coordinate(f64);

// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
impl PartialEq for Coordinate {
    fn eq(&self, other: &Coordinate) -> bool {
        (self.0 - other.0).abs() < f64::EPSILON
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Coordinate {
        Coordinate(self.0 + rhs.0)
    }
}

impl Sub for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Coordinate) -> Coordinate {
        Coordinate(self.0 - rhs.0)
    }
}

#[derive(Debug)]
struct Tuple(Coordinate, Coordinate, Coordinate);

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64) -> Tuple {
        let x = Coordinate(x);
        let y = Coordinate(y);
        let z = Coordinate(z);

        Tuple(x, y, z)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Tuple {
        let x = self.0 + rhs.0;
        let y = self.1 + rhs.1;
        let z = self.2 + rhs.2;

        Tuple::new(x.0, y.0, z.0)
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Tuple {
        let x = self.0 - rhs.0;
        let y = self.1 - rhs.1;
        let z = self.2 - rhs.2;

        Tuple::new(x.0, y.0, z.0)
    }
}

#[derive(Debug)]
struct Point {
    tuple: Tuple,
    w: Coordinate,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            tuple: Tuple::new(x, y, z),
            w: POINT_COORDINATE,
        }
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
        Point {
            tuple: self.tuple + rhs.tuple,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Vector {
        Vector {
            tuple: self.tuple - rhs.tuple,
            w: self.w - rhs.w,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Point {
        Point {
            tuple: self.tuple - rhs.tuple,
            w: self.w - rhs.w,
        }
    }
}

#[derive(Debug)]
struct Vector {
    tuple: Tuple,
    w: Coordinate,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector {
            tuple: Tuple::new(x, y, z),
            w: VECTOR_COORDINATE,
        }
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
        Vector {
            tuple: self.tuple + rhs.tuple,
            w: self.w + rhs.w,
        }
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            tuple: self.tuple + rhs.tuple,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            tuple: self.tuple - rhs.tuple,
            w: self.w - rhs.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparing_two_coordinates() {
        let c1 = Coordinate(1.0);
        let c2 = Coordinate(1.0);
        let c3 = Coordinate(1.0 + f64::EPSILON);

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
    fn creating_two_points() {
        let p = Point::new(1.0, 2.0, 3.0);

        assert_eq!(p.tuple, Tuple::new(1.0, 2.0, 3.0));
        assert_eq!(p.w, POINT_COORDINATE);
    }

    #[test]
    fn creating_two_vectors() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v.tuple, Tuple::new(1.0, 2.0, 3.0));
        assert_eq!(v.w, VECTOR_COORDINATE);
    }

    #[test]
    fn adding_two_coordiantes() {
        let c1 = Coordinate(1.0);
        let c2 = Coordinate(2.0);

        assert_eq!(c1 + c2, Coordinate(3.0));
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
}
