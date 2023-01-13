use crate::{
    float,
    intersections::Intersection,
    material::Material,
    ray::Ray,
    transform::Transform,
    tuple::{Point, Vector},
};

use super::Object;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Plane {
    pub material: Material,
    pub transform: Transform,
}

impl Plane {
    pub fn local_intersect(object: &Object, ray: Ray) -> Vec<Intersection<'_>> {
        if !float::approx(ray.direction.0.y, 0.0) {
            let t = -ray.origin.0.y / ray.direction.0.y;
            vec![Intersection { t, object }]
        } else {
            vec![]
        }
    }

    pub fn local_normal_at(_: Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;

    use super::*;

    fn test_plane_object() -> Object {
        Object::Plane(Default::default())
    }

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let n1 = Plane::local_normal_at(Point::new(0.0, 0.0, 0.0));
        let n2 = Plane::local_normal_at(Point::new(10.0, 0.0, -10.0));
        let n3 = Plane::local_normal_at(Point::new(-5.0, 0.0, 150.0));

        assert_eq!(n1, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n3, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = test_plane_object();

        let r = Ray {
            origin: Point::new(0.0, 10.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = Plane::local_intersect(&p, r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = test_plane_object();

        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = Plane::local_intersect(&p, r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = test_plane_object();

        let r = Ray {
            origin: Point::new(0.0, 1.0, 0.0),
            direction: Vector::new(0.0, -1.0, 0.0),
        };

        let xs = Plane::local_intersect(&p, r);

        assert_eq!(xs.len(), 1);
        assert_approx!(xs[0].t, 1.0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = test_plane_object();

        let r = Ray {
            origin: Point::new(0.0, -1.0, 0.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let xs = Plane::local_intersect(&p, r);

        assert_eq!(xs.len(), 1);
        assert_approx!(xs[0].t, 1.0);
    }
}
