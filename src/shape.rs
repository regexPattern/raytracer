use crate::intersection::Intersection;
use crate::material::Material;
use crate::ray::Ray;
use crate::transformation::Transformation;
use crate::tuple::{Point, Vector};

mod plane;
mod sphere;

pub use plane::Plane;
pub use sphere::Sphere;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
}

pub trait Intersectable {
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let ray = self.local_ray(ray);
        self.local_intersect(ray)
    }

    fn normal_at(&self, point: Point) -> Vector {
        let object_point = self.transform().inverse() * point;
        let object_normal = self.local_normal_at(object_point);
        let mut world_normal = self.transform().inverse().transpose() * object_normal;
        world_normal.0.w = 0.0;

        world_normal.normalize()
    }

    fn local_ray(&self, world_ray: Ray) -> Ray {
        world_ray.transform(self.transform().inverse())
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection>;

    fn local_normal_at(&self, object_point: Point) -> Vector;

    fn material(&self) -> Material;

    fn transform(&self) -> Transformation;
}

impl Intersectable for Shape {
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        match self {
            Shape::Sphere(s) => s.local_intersect(ray),
            Shape::Plane(p) => p.local_intersect(ray),
        }
    }

    fn local_normal_at(&self, object_point: Point) -> Vector {
        match self {
            Shape::Sphere(s) => s.local_normal_at(object_point),
            Shape::Plane(p) => p.local_normal_at(object_point),
        }
    }

    fn material(&self) -> Material {
        match self {
            Shape::Sphere(s) => s.material(),
            Shape::Plane(p) => p.material(),
        }
    }

    fn transform(&self) -> Transformation {
        match self {
            Shape::Sphere(s) => s.transform(),
            Shape::Plane(p) => p.transform(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::material::Material;
    use crate::transformation;

    struct TestShape {
        transform: Transformation,
        material: Material,
    }

    impl Default for TestShape {
        fn default() -> Self {
            Self {
                transform: Transformation::identity(),
                material: Material::default(),
            }
        }
    }

    impl Intersectable for TestShape {
        fn local_intersect(&self, _: Ray) -> Vec<Intersection> {
            Vec::new()
        }

        fn local_normal_at(&self, object_point: Point) -> Vector {
            Vector::new(object_point.0.x, object_point.0.y, object_point.0.z)
        }

        fn material(&self) -> Material {
            self.material
        }

        fn transform(&self) -> Transformation {
            self.transform
        }
    }

    #[test]
    fn the_default_transformation() {
        let s = TestShape::default();

        assert_eq!(s.transform(), Transformation::identity());
    }

    #[test]
    fn assigning_a_transformation() {
        let t = transformation::translation(2.0, 3.0, 4.0);
        let s = TestShape {
            transform: t,
            ..TestShape::default()
        };

        assert_eq!(s.transform(), t);
    }

    #[test]
    fn the_default_material() {
        let s = TestShape::default();

        assert_eq!(s.material(), Material::default());
    }

    #[test]
    fn assigning_a_material() {
        let m = Material {
            ambient: 1.0,
            ..Material::default()
        };

        let s = TestShape {
            material: m,
            ..TestShape::default()
        };

        assert_eq!(s.material, m);
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = TestShape {
            transform: transformation::scaling(2.0, 2.0, 2.0),
            ..TestShape::default()
        };

        let saved_ray = s.local_ray(r);

        assert_eq!(saved_ray.origin, Point::new(0.0, 0.0, -2.5));
        assert_eq!(saved_ray.direction, Vector::new(0.0, 0.0, 0.5));
    }

    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = TestShape {
            transform: transformation::translation(5.0, 0.0, 0.0),
            ..TestShape::default()
        };

        let saved_ray = s.local_ray(r);

        assert_eq!(saved_ray.origin, Point::new(-5.0, 0.0, -5.0));
        assert_eq!(saved_ray.direction, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let s = TestShape {
            transform: transformation::translation(0.0, 1.0, 0.0),
            ..TestShape::default()
        };

        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));

        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let s = TestShape {
            transform: transformation::scaling(1.0, 0.5, 1.0)
                * transformation::rotation_z(std::f64::consts::PI / 5.0),
            ..TestShape::default()
        };

        let n = s.normal_at(Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0));

        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }
}
