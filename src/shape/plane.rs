use crate::float;
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::tuple::{Point, Vector};

use super::{Figure, Shape};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Plane(pub Figure);

impl Plane {
    pub fn intersect(&self, object_ray: &Ray) -> Vec<Intersection> {
        let xs = Vec::new();

        if float::approx(object_ray.direction.0.y, 0.0) {
            return xs;
        }

        let t = -object_ray.origin.0.y / object_ray.direction.0.y;

        let i = Intersection {
            object: Shape::Plane(*self),
            t,
        };

        vec![i]
    }

    pub const fn normal_at(&self, _: Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;

    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let plane = Plane::default();

        let n1 = plane.normal_at(Point::new(0.0, 0.0, 0.0));
        let n2 = plane.normal_at(Point::new(10.0, 0.0, -10.0));
        let n3 = plane.normal_at(Point::new(-5.0, 0.0, 150.0));

        assert_eq!(n1, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n3, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let ray = Ray {
            origin: Point::new(0.0, 10.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let plane = Plane::default();

        let xs = plane.intersect(&ray);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let plane = Plane::default();

        let xs = plane.intersect(&ray);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let ray = Ray {
            origin: Point::new(0.0, 1.0, 0.0),
            direction: Vector::new(0.0, -1.0, 0.0),
        };

        let plane = Plane::default();

        let xs = plane.intersect(&ray);

        assert_eq!(xs.len(), 1);
        assert_approx!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, Shape::Plane(plane));
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let ray = Ray {
            origin: Point::new(0.0, -1.0, 0.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let plane = Plane::default();

        let xs = plane.intersect(&ray);

        assert_eq!(xs.len(), 1);
        assert_approx!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, Shape::Plane(plane));
    }
}
