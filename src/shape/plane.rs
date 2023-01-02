use crate::{
    ray::Ray,
    tuple::{Point, Vector},
    utils,
};

use super::Object;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Plane(pub Object);

impl Plane {
    pub(crate) fn local_intersect(&self, object_ray: &Ray) -> Vec<f64> {
        utils::approx(object_ray.direction.0.y, 0.0)
            .then_some(vec![])
            .unwrap_or(vec![-object_ray.origin.0.y / object_ray.direction.0.y])
    }

    pub(crate) fn local_normal_at(&self, _: Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;

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
    fn intersect_with_a_coplanar_ray() {
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
        assert_approx!(xs[0], 1.0);
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
        assert_approx!(xs[0], 1.0);
    }
}
