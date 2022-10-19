pub mod sphere;

pub use sphere::Sphere;

use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::{self, Matrix};
use crate::ray::Ray;
use crate::tuple::{Point, Vector};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Shape {
    pub material: Material,
    pub transform: Matrix<4, 4>,
}

impl Default for Shape {
    fn default() -> Self {
        Self {
            material: Material::default(),
            transform: matrix::IDENTITY4X4,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shapes {
    Sphere(Sphere),
}

impl Shapes {
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let local_ray = self.local_ray(ray);
        match self {
            Shapes::Sphere(s) => s.intersect(&local_ray),
        }
    }

    pub fn normal_at(&self, world_point: Point) -> Vector {
        let local_point = self.local_point(world_point);
        let local_normal = match self {
            Shapes::Sphere(s) => s.normal_at(local_point),
        };

        self.world_normal(local_normal)
    }

    fn local_ray(&self, ray: &Ray) -> Ray {
        ray.transform(self.transform().inverse())
    }

    fn local_point(&self, world_point: Point) -> Point {
        self.transform().inverse() * world_point
    }

    fn world_normal(&self, local_normal: Vector) -> Vector {
        let mut world_normal = self.transform().inverse().transpose() * local_normal;
        world_normal.0.w = 0.0;
        world_normal.normalize()
    }

    pub fn shape(&self) -> &Shape {
        match self {
            Shapes::Sphere(s) => &s.0,
        }
    }

    fn transform(&self) -> Matrix<4, 4> {
        match self {
            Shapes::Sphere(s) => s.0.transform,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_shape(transform: Matrix<4, 4>) -> Shapes {
        Shapes::Sphere(Sphere(Shape {
            transform,
            ..Default::default()
        }))
    }

    fn test_shape_normal_at(shape: &Shapes, point: Point) -> Vector {
        let local_point = shape.local_point(point);
        let local_normal = Vector::new(local_point.0.x, local_point.0.y, local_point.0.z);
        shape.world_normal(local_normal)
    }

    #[test]
    fn the_default_transformation() {
        let s = Shape::default();

        assert_eq!(s.transform, matrix::IDENTITY4X4);
    }

    #[test]
    fn assigning_a_transformation() {
        let mut s = Shape::default();
        let t = Matrix::translation(2.0, 3.0, 4.0);

        s.transform = t;

        assert_eq!(s.transform, t);
    }

    #[test]
    fn the_default_material() {
        let s = Shape::default();

        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn assigning_a_material() {
        let mut s = Shape::default();
        let mut m = Material::default();
        m.ambient = 1.0;

        s.material = m.clone();

        assert_eq!(s.material, m);
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = test_shape(Matrix::scaling(2.0, 2.0, 2.0));

        let saved_ray = s.local_ray(&r);

        assert_eq!(saved_ray.origin, Point::new(0.0, 0.0, -2.5));
        assert_eq!(saved_ray.direction, Vector::new(0.0, 0.0, 0.5));
    }

    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = test_shape(Matrix::translation(5.0, 0.0, 0.0));

        let saved_ray = s.local_ray(&r);

        assert_eq!(saved_ray.origin, Point::new(-5.0, 0.0, -5.0));
        assert_eq!(saved_ray.direction, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let s = test_shape(Matrix::translation(0.0, 1.0, 0.0));

        let n = test_shape_normal_at(&s, Point::new(0.0, 1.70711, -0.70711));

        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let s = test_shape(
            Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(std::f64::consts::PI / 5.0),
        );

        let n = test_shape_normal_at(&s, Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0));

        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }
}
