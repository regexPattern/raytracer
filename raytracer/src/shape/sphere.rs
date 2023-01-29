use crate::{
    intersection::Intersection,
    material::Material,
    ray::Ray,
    transform::Transform,
    tuple::{Point, Vector},
};

use super::{Bounds, Shape, ShapeProps};

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere(pub(crate) ShapeProps);

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl Sphere {
    pub fn new(material: Material, transform: Transform) -> Self {
        let local_bounds = Bounds {
            min: Point::new(-1.0, -1.0, -1.0),
            max: Point::new(1.0, 1.0, 1.0),
        };

        Self(ShapeProps {
            material,
            transform,
            transform_inverse: transform.inverse(),
            local_bounds,
            world_bounds: local_bounds.transform(transform),
        })
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.0.change_transform(transform);
        self
    }

    pub(crate) fn local_intersect<'a>(
        &self,
        object: &'a Shape,
        local_ray: &Ray,
    ) -> Vec<Intersection<'a>> {
        let ray_origin_vec = local_ray.origin - Point::new(0.0, 0.0, 0.0);

        let a = local_ray.direction.dot(local_ray.direction);
        let b = 2.0 * local_ray.direction.dot(ray_origin_vec);
        let c = ray_origin_vec.dot(ray_origin_vec) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        let t0 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![
            Intersection {
                t: t0,
                object,
                u: None,
                v: None,
            },
            Intersection {
                t: t1,
                object,
                u: None,
                v: None,
            },
        ]
    }

    pub(crate) fn local_normal_at(&self, local_point: Point) -> Vector {
        local_point - Point::new(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;

    use super::*;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let s = Sphere::default();
        let o = Shape::Sphere(Default::default());

        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = s.local_intersect(&o, &r);

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let s = Sphere::new(Default::default(), Default::default());
        let o = Shape::Sphere(Default::default());

        let r = Ray {
            origin: Point::new(0.0, 1.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = s.local_intersect(&o, &r);

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0].t, 5.0);
        assert_approx!(xs[1].t, 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let s = Sphere::default();
        let o = Shape::Sphere(Default::default());

        let r = Ray {
            origin: Point::new(0.0, 2.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = s.local_intersect(&o, &r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let s = Sphere::default();
        let o = Shape::Sphere(Default::default());

        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = s.local_intersect(&o, &r);

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0].t, -1.0);
        assert_approx!(xs[1].t, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let s = Sphere::default();
        let o = Shape::Sphere(Default::default());

        let r = Ray {
            origin: Point::new(0.0, 0.0, 5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = s.local_intersect(&o, &r);

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0].t, -6.0);
        assert_approx!(xs[1].t, -4.0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::default();

        let n = s.local_normal_at(Point::new(1.0, 0.0, 0.0));

        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::default();

        let n = s.local_normal_at(Point::new(0.0, 1.0, 0.0));

        assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::default();

        let n = s.local_normal_at(Point::new(0.0, 0.0, 1.0));

        assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_non_axial_point() {
        let s = Sphere::default();

        let n = s.local_normal_at(Point::new(
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
        ));

        assert_eq!(
            n,
            Vector::new(3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0)
        );
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::default();

        let n = s.local_normal_at(Point::new(
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
        ));

        assert_eq!(n, n.normalize().unwrap());
    }

    #[test]
    fn a_sphere_has_a_bounding_box() {
        let s = Sphere::default();
        let bounds = s.0.local_bounds;

        assert_eq!(bounds.min, Point::new(-1.0, -1.0, -1.0));
        assert_eq!(bounds.max, Point::new(1.0, 1.0, 1.0));
    }
}
