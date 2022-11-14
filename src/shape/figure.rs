use crate::{
    material::Material,
    matrix::{self, Matrix},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Figure {
    pub material: Material,
    pub transform: Matrix<4, 4>,
}

impl Default for Figure {
    fn default() -> Self {
        Self {
            material: Material::default(),
            transform: matrix::IDENTITY4X4,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ray::Ray,
        shape::{Shape, Sphere},
        tuple::{Point, Vector},
    };

    use super::*;

    fn test_shape(transform: Matrix<4, 4>) -> Shape {
        Shape::Sphere(Sphere(Figure {
            transform,
            ..Default::default()
        }))
    }

    fn test_shape_normal_at(shape: Shape, world_point: Point) -> Vector {
        let object_point = shape.object_point(world_point);
        let object_normal = Vector::new(object_point.0.x, object_point.0.y, object_point.0.z);
        shape.world_normal(object_normal)
    }

    #[test]
    fn the_default_transformation() {
        let shape = Figure::default();

        assert_eq!(shape.transform, matrix::IDENTITY4X4);
    }

    #[test]
    fn assigning_a_transformation() {
        let mut shape = Figure::default();
        let transform = Matrix::translation(2.0, 3.0, 4.0);

        shape.transform = transform;

        assert_eq!(shape.transform, transform);
    }

    #[test]
    fn the_default_material() {
        let shape = Figure::default();

        assert_eq!(shape.material, Material::default());
    }

    #[test]
    fn assigning_a_material() {
        let mut shape = Figure::default();
        let mut material = Material::default();
        material.ambient = 1.0;

        shape.material = material.clone();

        assert_eq!(shape.material, material);
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let shape = test_shape(Matrix::scaling(2.0, 2.0, 2.0));

        let saved_ray = shape.object_ray(&ray);

        assert_eq!(saved_ray.origin, Point::new(0.0, 0.0, -2.5));
        assert_eq!(saved_ray.direction, Vector::new(0.0, 0.0, 0.5));
    }

    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let shape = test_shape(Matrix::translation(5.0, 0.0, 0.0));

        let saved_ray = shape.object_ray(&ray);

        assert_eq!(saved_ray.origin, Point::new(-5.0, 0.0, -5.0));
        assert_eq!(saved_ray.direction, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let shape = test_shape(Matrix::translation(0.0, 1.0, 0.0));

        let n = test_shape_normal_at(shape, Point::new(0.0, 1.70711, -0.70711));

        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let shape = test_shape(
            Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(std::f64::consts::PI / 5.0),
        );

        let n = test_shape_normal_at(
            shape,
            Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0),
        );

        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }
}
