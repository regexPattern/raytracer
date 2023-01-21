use crate::{
    float,
    intersection::Intersection,
    ray::Ray,
    tuple::{Point, Vector},
};

use super::{BoundingBox, Shape};

pub fn intersect<'a>(object: &'a Shape, ray: &Ray) -> Vec<Intersection<'a>> {
    if !float::approx(ray.direction.0.y, 0.0) {
        let t = -ray.origin.0.y / ray.direction.0.y;
        vec![Intersection { t, object }]
    } else {
        vec![]
    }
}

pub fn normal_at(_: Point) -> Vector {
    Vector::new(0.0, 1.0, 0.0)
}

pub fn bounding_box() -> BoundingBox {
    BoundingBox {
        min: Point::new(std::f64::NEG_INFINITY, 0.0, std::f64::NEG_INFINITY),
        max: Point::new(std::f64::INFINITY, 0.0, std::f64::INFINITY),
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;

    use super::*;

    fn dummy_object() -> Shape {
        Shape::Plane(Default::default())
    }

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let n0 = super::normal_at(Point::new(0.0, 0.0, 0.0));
        let n1 = super::normal_at(Point::new(10.0, 0.0, -10.0));
        let n2 = super::normal_at(Point::new(-5.0, 0.0, 150.0));

        assert_eq!(n0, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n1, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let o = dummy_object();

        let r = Ray {
            origin: Point::new(0.0, 10.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = super::intersect(&o, &r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let o = dummy_object();

        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = super::intersect(&o, &r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let o = dummy_object();

        let r = Ray {
            origin: Point::new(0.0, 1.0, 0.0),
            direction: Vector::new(0.0, -1.0, 0.0),
        };

        let xs = super::intersect(&o, &r);

        assert_eq!(xs.len(), 1);
        assert_approx!(xs[0].t, 1.0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let o = dummy_object();

        let r = Ray {
            origin: Point::new(0.0, -1.0, 0.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let xs = super::intersect(&o, &r);

        assert_eq!(xs.len(), 1);
        assert_approx!(xs[0].t, 1.0);
    }

    #[test]
    fn a_plane_has_a_bounding_box() {
        let bbox = super::bounding_box();

        assert_eq!(
            bbox.min,
            Point::new(std::f64::NEG_INFINITY, 0.0, std::f64::NEG_INFINITY)
        );
        assert_eq!(
            bbox.max,
            Point::new(std::f64::INFINITY, 0.0, std::f64::INFINITY)
        );
    }
}
