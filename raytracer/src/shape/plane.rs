use crate::{
    float,
    intersection::Intersection,
    ray::Ray,
    tuple::{Point, Vector},
};

use super::{bounding_box::BoundingBox, object::ObjectCache, Shape, ShapeBuilder};

/// Representation of a plane.
///
/// Must be built from a [ShapeBuilder].
#[derive(Clone, Debug, PartialEq)]
pub struct Plane(pub(crate) ObjectCache);

impl Default for Plane {
    fn default() -> Self {
        Self::from(ShapeBuilder::default())
    }
}

impl From<ShapeBuilder> for Plane {
    fn from(builder: ShapeBuilder) -> Self {
        let ShapeBuilder {
            material,
            transform,
        } = builder;

        let bounding_box = BoundingBox {
            min: Point::new(std::f64::NEG_INFINITY, 0.0, std::f64::NEG_INFINITY),
            max: Point::new(std::f64::INFINITY, 0.0, std::f64::INFINITY),
        };

        Self(ObjectCache::new(material, transform, bounding_box))
    }
}

impl Plane {
    pub(crate) fn intersect<'a>(&self, object: &'a Shape, ray: &Ray) -> Vec<Intersection<'a>> {
        if !float::approx(ray.direction.0.y, 0.0) {
            let t = -ray.origin.0.y / ray.direction.0.y;
            vec![Intersection {
                t,
                object,
                u: None,
                v: None,
            }]
        } else {
            vec![]
        }
    }

    pub(crate) fn normal_at(&self, _: Point) -> Vector {
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

        let n0 = plane.normal_at(Point::new(0.0, 0.0, 0.0));
        let n1 = plane.normal_at(Point::new(10.0, 0.0, -10.0));
        let n2 = plane.normal_at(Point::new(-5.0, 0.0, 150.0));

        assert_eq!(n0, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n1, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let plane = Plane::default();
        let object = Shape::Plane(Default::default());

        let ray = Ray {
            origin: Point::new(0.0, 10.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = plane.intersect(&object, &ray);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let plane = Plane::default();
        let object = Shape::Plane(Default::default());

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = plane.intersect(&object, &ray);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let plane = Plane::default();
        let object = Shape::Plane(Default::default());

        let ray = Ray {
            origin: Point::new(0.0, 1.0, 0.0),
            direction: Vector::new(0.0, -1.0, 0.0),
        };

        let xs = plane.intersect(&object, &ray);

        assert_eq!(xs.len(), 1);
        assert_approx!(xs[0].t, 1.0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let plane = Plane::default();
        let object = Shape::Plane(Default::default());

        let ray = Ray {
            origin: Point::new(0.0, -1.0, 0.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let xs = plane.intersect(&object, &ray);

        assert_eq!(xs.len(), 1);
        assert_approx!(xs[0].t, 1.0);
    }

    #[test]
    fn a_plane_has_a_bounding_box() {
        let plane = Plane::default();
        let bounding_box = plane.0.bounding_box;

        assert_eq!(
            bounding_box.min,
            Point::new(std::f64::NEG_INFINITY, 0.0, std::f64::NEG_INFINITY)
        );
        assert_eq!(
            bounding_box.max,
            Point::new(std::f64::INFINITY, 0.0, std::f64::INFINITY)
        );
    }
}
