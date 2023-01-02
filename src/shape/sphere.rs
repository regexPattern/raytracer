use crate::{
    ray::Ray,
    tuple::{Point, Vector},
};

use super::Object;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sphere(pub Object);

impl Sphere {
    pub(crate) fn local_intersect(&self, object_ray: &Ray) -> Vec<f64> {
        let ray_origin_vec = object_ray.origin - Point::new(0.0, 0.0, 0.0);

        let a = object_ray.direction.dot(object_ray.direction);
        let b = 2.0 * object_ray.direction.dot(ray_origin_vec);
        let c = ray_origin_vec.dot(ray_origin_vec) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec::new();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![t1, t2]
    }

    pub(crate) fn local_normal_at(&self, object_point: Point) -> Vector {
        object_point - Point::new(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_approx,
        tuple::{Point, Vector},
    };

    use super::*;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Sphere::default();

        let xs = s.local_intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0], 4.0);
        assert_approx!(xs[1], 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray {
            origin: Point::new(0.0, 1.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Sphere::default();

        let xs = s.local_intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0], 5.0);
        assert_approx!(xs[1], 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray {
            origin: Point::new(0.0, 2.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Sphere::default();

        let xs = s.local_intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Sphere::default();

        let xs = s.local_intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0], -1.0);
        assert_approx!(xs[1], 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, 5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Sphere::default();

        let xs = s.local_intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0], -6.0);
        assert_approx!(xs[1], -4.0);
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
}
