use crate::float;
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::tuple::{Point, Vector};

use super::{Figure, Shapes};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Plane(pub Figure);

impl Plane {
    pub fn local_intersect(self, ray: &Ray) -> Vec<Intersection> {
        let xs = Vec::new();

        if float::approx(ray.direction.0.y, 0.0) {
            return xs;
        }

        let t = -ray.origin.0.y / ray.direction.0.y;

        let i = Intersection {
            object: Shapes::Plane(self),
            t,
        };

        vec![i]
    }

    pub fn local_normal_at(&self, _: Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_approx, shape::Shapes};

    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::default();

        let n1 = p.local_normal_at(Point::new(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(Point::new(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(Point::new(-5.0, 0.0, 150.0));

        assert_eq!(n1, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n3, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane::default();
        let r = Ray {
            origin: Point::new(0.0, 10.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = p.local_intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let p = Plane::default();
        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = p.local_intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Plane::default();
        let r = Ray {
            origin: Point::new(0.0, 1.0, 0.0),
            direction: Vector::new(0.0, -1.0, 0.0),
        };

        let xs = p.local_intersect(&r);

        assert_eq!(xs.len(), 1);
        assert_approx!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, Shapes::Plane(p));
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::default();
        let r = Ray {
            origin: Point::new(0.0, -1.0, 0.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let xs = p.local_intersect(&r);

        assert_eq!(xs.len(), 1);
        assert_approx!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, Shapes::Plane(p));
    }
}
