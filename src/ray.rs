use crate::matrix::Matrix;
use crate::tuple::{Point, Vector};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn transform(&self, transform: Matrix<4, 4>) -> Self {
        let origin = transform * self.origin;
        let direction = transform * self.direction;

        Self { origin, direction }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector::new(4.0, 5.0, 6.0);

        let ray = Ray { origin, direction };

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let ray = Ray {
            origin: Point::new(2.0, 3.0, 4.0),
            direction: Vector::new(1.0, 0.0, 0.0),
        };

        assert_eq!(ray.position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn translating_a_ray() {
        let ray = Ray {
            origin: Point::new(1.0, 2.0, 3.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let transform = Matrix::translation(3.0, 4.0, 5.0);

        let r2 = ray.transform(transform);

        assert_eq!(r2.origin, Point::new(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let ray = Ray {
            origin: Point::new(1.0, 2.0, 3.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let transform = Matrix::scaling(2.0, 3.0, 4.0);

        let r2 = ray.transform(transform);

        assert_eq!(r2.origin, Point::new(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Vector::new(0.0, 3.0, 0.0));
    }
}
