use crate::intersection::Intersection;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::{Intersectable, Shape};
use crate::transformation::Transformation;
use crate::tuple::{Point, Vector};
use crate::utils;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Plane {
    material: Material,
    transform: Transformation,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            transform: Transformation::identity(),
            material: Material::default(),
        }
    }
}

impl Intersectable for Plane {
    fn local_normal_at(&self, _: Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        if utils::approximately_eq(ray.direction.0.y, 0.0) {
            return Vec::new();
        }

        let t = -ray.origin.0.y / ray.direction.0.y;

        vec![Intersection::new(t, Shape::Plane(*self))]
    }

    fn material(&self) -> Material {
        self.material
    }

    fn transform(&self) -> Transformation {
        self.transform
    }
}

#[cfg(test)]
mod tests {
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
        let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));

        let xs = p.local_intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Plane::default();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));

        let xs = p.local_intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Plane::default();
        let r = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));

        let xs = p.local_intersect(r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, Shape::Plane(p));
    }

    #[test]
    fn a_ray_intersects_a_plane_from_below() {
        let p = Plane::default();
        let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));

        let xs = p.local_intersect(r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, Shape::Plane(p));
    }
}
