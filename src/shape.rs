use crate::{
    intersection::Intersection,
    ray::Ray,
    tuple::{Point, Vector},
};

mod figure;
mod plane;
mod sphere;

pub use figure::Figure;
pub use plane::Plane;
pub use sphere::Sphere;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    Plane(Plane),
    Sphere(Sphere),
}

impl Shape {
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let object_ray = self.object_ray(ray);
        match self {
            Self::Plane(p) => p.intersect(&object_ray),
            Self::Sphere(s) => s.intersect(&object_ray),
        }
    }

    pub fn normal_at(&self, world_point: Point) -> Vector {
        let object_point = self.object_point(world_point);
        let object_normal = match self {
            Self::Plane(p) => p.normal_at(object_point),
            Self::Sphere(s) => s.normal_at(object_point),
        };

        self.world_normal(object_normal)
    }

    fn object_ray(&self, ray: &Ray) -> Ray {
        ray.transform(self.shape().transform.inverse())
    }

    fn object_point(&self, world_point: Point) -> Point {
        self.shape().transform.inverse() * world_point
    }

    fn world_normal(&self, object_normal: Vector) -> Vector {
        let mut world_normal = self.shape().transform.inverse().transpose() * object_normal;
        world_normal.0.w = 0.0;
        world_normal.normalize()
    }

    pub const fn shape(&self) -> Figure {
        match self {
            Self::Plane(p) => p.0,
            Self::Sphere(s) => s.0,
        }
    }
}
