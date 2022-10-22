use crate::float;
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::tuple::{Point, Vector};

use super::{ShapeProps, Shape};

pub fn intersect(shape: ShapeProps, ray: Ray) -> Vec<Intersection> {
    let xs = Vec::new();

    if float::approx(ray.direction.0.y, 0.0) {
        return xs;
    }

    let t = -ray.origin.0.y / ray.direction.0.y;

    let i = Intersection {
        object: Shape::Plane(shape),
        t,
    };

    vec![i]
}

pub const fn normal_at(_: ShapeProps, _: Point) -> Vector {
    Vector::new(0.0, 1.0, 0.0)
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;
    use crate::shape::Shape;

    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = ShapeProps::default();

        let n1 = super::normal_at(p, Point::new(0.0, 0.0, 0.0));
        let n2 = super::normal_at(p, Point::new(10.0, 0.0, -10.0));
        let n3 = super::normal_at(p, Point::new(-5.0, 0.0, 150.0));

        assert_eq!(n1, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n3, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = ShapeProps::default();

        let r = Ray {
            origin: Point::new(0.0, 10.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = super::intersect(p, r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let p = ShapeProps::default();

        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = super::intersect(p, r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = ShapeProps::default();

        let r = Ray {
            origin: Point::new(0.0, 1.0, 0.0),
            direction: Vector::new(0.0, -1.0, 0.0),
        };

        let xs = super::intersect(p, r);

        assert_eq!(xs.len(), 1);
        assert_approx!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, Shape::Plane(p));
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = ShapeProps::default();

        let r = Ray {
            origin: Point::new(0.0, -1.0, 0.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let xs = super::intersect(p, r);

        assert_eq!(xs.len(), 1);
        assert_approx!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, Shape::Plane(p));
    }
}
