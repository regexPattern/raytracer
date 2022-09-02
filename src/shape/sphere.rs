use crate::lighting::{Intersection, Ray};
use crate::material::Material;
use crate::matrix::{Matrix, MATRIX_4X4};
use crate::tuple::Tuple;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    pub transform: Matrix<4, 4>,
    pub material: Material,
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(MATRIX_4X4.identity(), Material::default())
    }
}

impl From<Matrix<4, 4>> for Sphere {
    fn from(transform: Matrix<4, 4>) -> Self {
        Self::new(transform, Material::default())
    }
}

impl From<Material> for Sphere {
    fn from(material: Material) -> Self {
        Self::new(MATRIX_4X4.identity(), material)
    }
}

impl Sphere {
    fn new(transform: Matrix<4, 4>, material: Material) -> Self {
        Self {
            transform,
            material,
        }
    }

    pub fn intersect(self, ray: Ray) -> Vec<Intersection> {
        let ray = ray.transform(self.transform.inverse());

        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);

        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec::new();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![Intersection::new(t1, self), Intersection::new(t2, self)]
    }

    pub fn normal_at(self, point: Tuple) -> Tuple {
        let object_point = self.transform.inverse() * point;
        let object_normal = object_point - Tuple::point(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.w = 0.0;

        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::canvas::Color;
    use crate::matrix::transformation;
    use crate::tuple::Tuple;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn a_spheres_default_transformation() {
        let s = Sphere::default();

        assert_eq!(s.transform, MATRIX_4X4.identity());
    }

    #[test]
    fn changing_a_spheres_default_transformation() {
        let mut s = Sphere::default();
        let t = transformation::translation(2.0, 3.0, 4.0);

        s.transform = t;

        assert_eq!(s.transform, t);
    }

    #[test]
    fn constructing_a_sphere_from_a_transformation() {
        let m = transformation::translation(1.0, 1.0, 1.0);

        let s = Sphere::from(m);

        assert_eq!(s.transform, m);
        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn constructing_a_sphere_from_a_material() {
        let mut m = Material::default();
        m.color = Color::new(1.0, 0.0, 0.0);

        let s = Sphere::from(m);

        assert_eq!(s.material, m);
        assert_eq!(s.transform, MATRIX_4X4.identity());
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::default();

        s.transform = transformation::scaling(2.0, 2.0, 2.0);
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::default();

        s.transform = transformation::translation(5.0, 0.0, 0.0);
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::default();

        let n = s.normal_at(Tuple::point(1.0, 0.0, 0.0));

        assert_eq!(n, Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::default();

        let n = s.normal_at(Tuple::point(0.0, 1.0, 0.0));

        assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::default();

        let n = s.normal_at(Tuple::point(0.0, 0.0, 1.0));

        assert_eq!(n, Tuple::vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_at_a_nonaxial_point() {
        let s = Sphere::default();

        let n = s.normal_at(Tuple::point(
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
        ));

        assert_eq!(
            n,
            Tuple::vector(3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0,)
        );
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::default();

        let n = s.normal_at(Tuple::point(
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
        ));

        assert_eq!(n, n.normalize());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = Sphere::default();
        s.transform = transformation::translation(0.0, 1.0, 0.0);

        let n = s.normal_at(Tuple::point(0.0, 1.70711, -0.70711));

        assert_eq!(n, Tuple::vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = Sphere::default();
        let m = transformation::scaling(1.0, 0.5, 1.0)
            * transformation::rotation_z(std::f64::consts::PI / 5.0);
        s.transform = m;

        let n = s.normal_at(Tuple::point(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0));

        assert_eq!(n, Tuple::vector(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = Sphere::default();

        let m = s.material;

        assert_eq!(m, Material::default());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut s = Sphere::default();
        let mut m = Material::default();
        m.ambient = 1.0;

        s.material = m;

        assert_eq!(s.material, m);
    }
}
