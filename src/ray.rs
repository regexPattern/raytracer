use crate::transformation::Transformation;
use crate::tuple::{Point, Vector};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn position(self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn transform(self, m: Transformation) -> Self {
        let origin = m * self.origin;
        let direction = m * self.direction;
        Self::new(origin, direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::transformation;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector::new(4.0, 5.0, 6.0);

        let r = Ray::new(origin, direction);

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

        assert_eq!(r.position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = transformation::translation(3.0, 4.0, 5.0);

        let r2 = r.transform(m);

        assert_eq!(r2.origin, Point::new(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = transformation::scaling(2.0, 3.0, 4.0);

        let r2 = r.transform(m);

        assert_eq!(r2.origin, Point::new(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Vector::new(0.0, 3.0, 0.0));
    }
}
