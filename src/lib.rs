#![allow(dead_code)]

#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

#[derive(Debug, PartialEq)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }
}

impl Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_has_desired_coordinates() {
        let point = Point {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 1.0);
    }

    #[test]
    fn vector_has_desired_coordinates() {
        let vector = Vector {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };
        assert_eq!(vector.x, 4.3);
        assert_eq!(vector.y, -4.2);
        assert_eq!(vector.z, 3.1);
        assert_eq!(vector.w, 0.0);
    }

    #[test]
    fn point_constructor_creates_point() {
        let point = Point::new(4.3, -4.2, 3.1);
        assert_eq!(point, Point::new(4.3, -4.2, 3.1));
    }

    #[test]
    fn vector_constructor_creates_vector() {
        let vector = Point::new(4.3, -4.2, 3.1);
        assert_eq!(vector, Point::new(4.3, -4.2, 3.1));
    }
}
