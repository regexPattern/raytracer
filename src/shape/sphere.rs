use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix;
use crate::ray::Ray;
use crate::tuple::{Point, Vector};

use super::{Figure, Shapes};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere(pub Figure);

impl Default for Sphere {
    fn default() -> Self {
        Self(Figure {
            material: Material::default(),
            transform: matrix::IDENTITY4X4,
        })
    }
}

impl Sphere {
    // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
    pub fn local_intersect(self, ray: &Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - Point::new(0.0, 0.0, 0.0);

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec::new();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let i1 = Intersection {
            object: Shapes::Sphere(self),
            t: t1,
        };
        let i2 = Intersection {
            object: Shapes::Sphere(self),
            t: t2,
        };

        vec![i1, i2]
    }

    pub fn local_normal_at(&self, object_point: Point) -> Vector {
        object_point - Point::new(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;
    use crate::matrix::Matrix;
    use crate::tuple::Vector;

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
        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Shapes::Sphere(Sphere::default());

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
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
        assert_approx!(xs[0].t, 5.0);
        assert_approx!(xs[1].t, 5.0);
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
        assert_approx!(xs[0].t, -1.0);
        assert_approx!(xs[1].t, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, 5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Shapes::Sphere(Sphere::default());

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0].t, -6.0);
        assert_approx!(xs[1].t, -4.0);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Shapes::Sphere(Sphere(Figure {
            transform: Matrix::scaling(2.0, 2.0, 2.0),
            ..Default::default()
        }));

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0].t, 3.0);
        assert_approx!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Shapes::Sphere(Sphere(Figure {
            transform: Matrix::translation(5.0, 0.0, 0.0),
            ..Default::default()
        }));

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Shapes::Sphere(Sphere::default());

        let n = s.normal_at(Point::new(1.0, 0.0, 0.0));

        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Shapes::Sphere(Sphere::default());

        let n = s.normal_at(Point::new(0.0, 1.0, 0.0));

        assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Shapes::Sphere(Sphere::default());

        let n = s.normal_at(Point::new(0.0, 0.0, 1.0));

        assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Shapes::Sphere(Sphere::default());

        let n = s.normal_at(Point::new(
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
        let s = Shapes::Sphere(Sphere::default());

        let n = s.normal_at(Point::new(
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
        ));

        assert_eq!(n, n.normalize());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let s = Shapes::Sphere(Sphere(Figure {
            transform: Matrix::translation(0.0, 1.0, 0.0),
            ..Default::default()
        }));

        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));

        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let s = Shapes::Sphere(Sphere(Figure {
            transform: Matrix::scaling(1.0, 0.5, 1.0)
                * Matrix::rotation_z(std::f64::consts::PI / 5.0),
            ..Default::default()
        }));

        let n = s.normal_at(Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0));

        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }
}
