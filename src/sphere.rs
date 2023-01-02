use crate::{
    intersections::Intersection,
    material::Material,
    ray::Ray,
    transform::Transform,
    tuple::{Point, Vector},
};

#[derive(Debug, Default, PartialEq)]
pub struct Sphere {
    pub transform: Transform,
    pub material: Material,
}

impl Sphere {
    // · o : ray.origin (vector)
    // · u : ray.direction
    // · unit sphere : || x ||^2 = 1
    // · ray : x = o + tu
    //
    // || o + tu ||^2 = 1
    // (o + tu) · (o + tu) - 1 = 0
    // (u · u)t^2 + (2(o · u))t + (o · o - 1) = 0
    //    a             b              c
    //
    //  t = ?
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection<'_>> {
        let ray = ray.transform(self.transform.inverse());
        let ray_origin_vec = ray.origin - Point::new(0.0, 0.0, 0.0);

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(ray_origin_vec);
        let c = ray_origin_vec.dot(ray_origin_vec) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        // non-real solutions
        if discriminant < 0.0 {
            return Vec::new();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let object = self;
        let i1 = Intersection { t: t1, object };
        let i2 = Intersection { t: t2, object };

        vec![i1, i2]
    }

    // This function is only going to be used internally so the points it receives are always
    // garanteed to be on the sphere's surface, thus the normal will be always a valid, non-null
    // normalizable vector.
    pub fn normal_at(&self, point: Point) -> Vector {
        let object_point = self.transform.inverse() * point;
        let object_normal = object_point - Point::new(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.0.w = 0.0;

        #[allow(clippy::unwrap_used)]
        world_normal.normalize().unwrap()
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

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray {
            origin: Point::new(0.0, 1.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Sphere::default();

        let xs = s.intersect(&r);

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

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Sphere::default();

        let xs = s.intersect(&r);

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

        let s = Sphere::default();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0].t, -6.0);
        assert_approx!(xs[1].t, -4.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Sphere::default();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, &s);
        assert_eq!(xs[1].object, &s);
    }

    #[test]
    fn a_spheres_default_transformation() {
        let s = Sphere::default();

        assert_eq!(s.transform, Transform::default());
    }

    #[test]
    fn changing_a_sphere_s_transformation() {
        let transform = Transform::translation(2.0, 3.0, 4.0);

        let s = Sphere {
            transform,
            ..Default::default()
        };

        assert_eq!(s.transform, transform);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Sphere {
            transform: Transform::try_scaling(2.0, 2.0, 2.0).unwrap(),
            ..Default::default()
        };

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

        let s = Sphere {
            transform: Transform::translation(5.0, 0.0, 0.0),
            ..Default::default()
        };

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::default();

        let n = s.normal_at(Point::new(1.0, 0.0, 0.0));

        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::default();

        let n = s.normal_at(Point::new(0.0, 1.0, 0.0));

        assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::default();

        let n = s.normal_at(Point::new(0.0, 0.0, 1.0));

        assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_non_axial_point() {
        let s = Sphere::default();

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
        let s = Sphere::default();

        let n = s.normal_at(Point::new(
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
        ));

        assert_eq!(n, n.normalize().unwrap());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let s = Sphere {
            transform: Transform::translation(0.0, 1.0, 0.0),
            ..Default::default()
        };

        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));

        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let s = Sphere {
            transform: Transform::try_scaling(1.0, 0.5, 1.0).unwrap()
                * Transform::rotation_z(std::f64::consts::PI / 5.0),
            ..Default::default()
        };

        let n = s.normal_at(Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0));

        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = Sphere::default();

        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let material = Material {
            ambient: 1.0,
            ..Default::default()
        };

        let s = Sphere {
            material: material.clone(),
            ..Default::default()
        };

        assert_eq!(s.material, material);
    }
}
