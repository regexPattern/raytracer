use crate::{
    intersection::Intersection,
    ray::Ray,
    tuple::{Point, Vector},
    shape::{Figure, Shape},
};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Sphere(pub Figure);

impl Sphere {
    pub fn intersect(&self, object_ray: &Ray) -> Vec<Intersection> {
        let sphere_to_ray = object_ray.origin - Point::new(0.0, 0.0, 0.0);

        let a = object_ray.direction.dot(object_ray.direction);
        let b = 2.0 * object_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec::new();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let i1 = Intersection {
            object: Shape::Sphere(*self),
            t: t1,
        };
        let i2 = Intersection {
            object: Shape::Sphere(*self),
            t: t2,
        };

        vec![i1, i2]
    }

    pub fn normal_at(&self, object_point: Point) -> Vector {
        object_point - Point::new(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_approx, tuple::Vector};

    use super::*;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let sphere = Sphere::default();

        let xs = sphere.intersect(&ray);

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let sphere = Sphere::default();

        let xs = sphere.intersect(&ray);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, Shape::Sphere(sphere));
        assert_eq!(xs[1].object, Shape::Sphere(sphere));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let ray = Ray {
            origin: Point::new(0.0, 1.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let sphere = Sphere::default();

        let xs = sphere.intersect(&ray);

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0].t, 5.0);
        assert_approx!(xs[1].t, 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let ray = Ray {
            origin: Point::new(0.0, 2.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let sphere = Sphere::default();

        let xs = sphere.intersect(&ray);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let sphere = Sphere::default();

        let xs = sphere.intersect(&ray);

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0].t, -1.0);
        assert_approx!(xs[1].t, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, 5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let sphere = Sphere::default();

        let xs = sphere.intersect(&ray);

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0].t, -6.0);
        assert_approx!(xs[1].t, -4.0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let sphere = Sphere::default();

        let n = sphere.normal_at(Point::new(1.0, 0.0, 0.0));

        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let sphere = Sphere::default();

        let n = sphere.normal_at(Point::new(0.0, 1.0, 0.0));

        assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let sphere = Sphere::default();

        let n = sphere.normal_at(Point::new(0.0, 0.0, 1.0));

        assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let sphere = Sphere::default();

        let n = sphere.normal_at(Point::new(
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
        let sphere = Sphere::default();

        let n = sphere.normal_at(Point::new(
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
        ));

        assert_eq!(n, n.normalize());
    }
}
