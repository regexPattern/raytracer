use crate::lighting::Material;
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
        let shape_center = Tuple::point(0.0, 0.0, 0.0);

        let object_point = self.transformation.inverse() * world_point;
        let object_normal = object_point - shape_center;
        let mut world_normal = self.transformation.inverse().transpose() * object_normal;
        world_normal.w = 0.0;

        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::canvas::Color;
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
}
