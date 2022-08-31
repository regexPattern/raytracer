use crate::lighting::{Intersection, Material, Ray};
use crate::matrix::{self, Matrix};
use crate::tuple::Tuple;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    pub transformation: Matrix<4, 4>,
    pub material: Material,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            transformation: matrix::MATRIX_4X4.identity(),
            material: Material::default(),
        }
    }
}

impl From<Matrix<4, 4>> for Sphere {
    fn from(transformation: Matrix<4, 4>) -> Self {
        Self::new(transformation, Material::default())
    }
}

impl From<Material> for Sphere {
    fn from(material: Material) -> Self {
        Self::new(matrix::MATRIX_4X4.identity(), material)
    }
}

impl Sphere {
    pub fn new(transformation: Matrix<4, 4>, material: Material) -> Self {
        Sphere {
            transformation,
            material,
        }
    }

    pub fn normal_at(self, world_point: Tuple) -> Tuple {
        let object_point = self.transformation.inverse() * world_point;
        let object_normal = object_point - Tuple::point(0.0, 0.0, 0.0);
        let mut world_normal = self.transformation.inverse().transpose() * object_normal;
        world_normal.w = 0.0;

        world_normal.normalize()
    }

    pub fn intersect(self, ray: Ray) -> Vec<Intersection> {
        let ray = ray.transform(self.transformation.inverse());

        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);

        // https://gregorycernera.medium.com/an-explanation-of-basic-ray-tracing-313373c852ac#:~:text=If%20the%20discriminant%20is%20negative,will%20be%20two%20hit%20points.
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
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::canvas::Color;
    use crate::lighting::Ray;
    use crate::matrix::transformation;

    #[test]
    fn creating_default_sphere() {
        let s = Sphere::default();

        assert_eq!(s.transformation, matrix::MATRIX_4X4.identity());
        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn creating_sphere_from_transformation_has_a_default_material() {
        let t = transformation::scaling(1.0, 1.0, 1.0);
        let s = Sphere::from(t);

        assert_eq!(s.transformation, t);
        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn creating_sphere_from_material_has_a_default_transformation() {
        let mut m = Material::default();
        m.color = Color::new(1.0, 0.2, 1.0);

        let s = Sphere::from(m);

        assert_eq!(s.material, m);
        assert_eq!(s.transformation, matrix::MATRIX_4X4.identity());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut s = Sphere::default();
        let mut m = Material::default();
        m.ambient = 1.0;

        s.material = m;

        assert_eq!(s.material, m);
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
    fn the_normal_on_a_sphere_at_a_non_axial_point() {
        let s = Sphere::default();

        let n = s.normal_at(Tuple::point(
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
        ));

        assert_eq!(
            n,
            Tuple::vector(3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0)
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
        s.transformation = transformation::translation(0.0, 1.0, 0.0);

        let n = s.normal_at(Tuple::point(0.0, 1.70711, -0.70711));

        assert_eq!(n, Tuple::vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = Sphere::default();
        s.transformation = transformation::scaling(1.0, 0.5, 1.0)
            * transformation::rotation_z(std::f64::consts::PI / 5.0);

        let n = s.normal_at(Tuple::point(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0));

        assert_eq!(n, Tuple::vector(0.0, 0.97014, -0.24254));
    }

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
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::default();

        s.transformation = transformation::scaling(2.0, 2.0, 2.0);
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::default();

        s.transformation = transformation::translation(5.0, 0.0, 0.0);
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }
}
