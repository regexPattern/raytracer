mod plane;
mod sphere;

use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::{self, Matrix};
use crate::ray::Ray;
use crate::tuple::{Point, Vector};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shapes {
    Sphere(Shape),
    Plane(Shape),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Shape {
    pub transform: Matrix<4, 4>,
    pub material: Material,
}

impl Default for Shape {
    fn default() -> Self {
        Self {
            material: Material::default(),
            transform: matrix::IDENTITY4X4,
        }
    }
}

impl Shapes {
    pub fn intersect(self, ray: Ray) -> Vec<Intersection> {
        let local_ray = self.local_ray(ray);
        match self {
            Shapes::Sphere(s) => sphere::intersect(s, local_ray),
            Shapes::Plane(p) => plane::intersect(p, local_ray),
        }
    }

    pub fn normal_at(self, world_point: Point) -> Vector {
        let local_point = self.local_point(world_point);
        let local_normal = match self {
            Shapes::Sphere(s) => sphere::normal_at(s, local_point),
            Shapes::Plane(p) => plane::normal_at(p, local_point),
        };

        self.world_normal(local_normal)
    }

    fn local_ray(&self, ray: Ray) -> Ray {
        ray.transform(self.shape().transform.inverse())
    }

    fn local_point(&self, world_point: Point) -> Point {
        self.shape().transform.inverse() * world_point
    }

    fn world_normal(&self, local_normal: Vector) -> Vector {
        let mut world_normal = self.shape().transform.inverse().transpose() * local_normal;
        world_normal.0.w = 0.0;
        world_normal.normalize()
    }

    pub fn shape(self) -> Shape {
        match self {
            Shapes::Sphere(s) => s,
            Shapes::Plane(p) => p,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_shape(transform: Matrix<4, 4>) -> Shapes {
        Shapes::Sphere(Shape {
            transform,
            ..Default::default()
        })
    }

    fn test_shape_normal_at(shape: Shapes, point: Point) -> Vector {
        let local_point = shape.local_point(point);
        let local_normal = Vector::new(local_point.0.x, local_point.0.y, local_point.0.z);
        shape.world_normal(local_normal)
    }

    #[test]
    fn the_default_transformation() {
        let shape = Shape::default();

        assert_eq!(shape.transform, matrix::IDENTITY4X4);
    }

    #[test]
    fn assigning_a_transformation() {
        let mut shape = Shape::default();
        let transform = Matrix::translation(2.0, 3.0, 4.0);

        shape.transform = transform;

        assert_eq!(shape.transform, transform);
    }

    #[test]
    fn the_default_material() {
        let shape = Shape::default();

        assert_eq!(shape.material, Material::default());
    }

    #[test]
    fn assigning_a_material() {
        let mut shape = Shape::default();
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

        let saved_ray = shape.local_ray(ray);

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

        let saved_ray = shape.local_ray(ray);

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

        let n = test_shape_normal_at(shape, Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0));

        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }
}
