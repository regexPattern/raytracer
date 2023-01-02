use crate::{
    intersections::Intersection,
    material::Material,
    ray::Ray,
    transform::Transform,
    tuple::{Point, Vector},
};

mod plane;
mod sphere;

pub use plane::Plane;
pub use sphere::Sphere;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Object {
    pub transform: Transform,
    pub material: Material,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Plane(Plane),
    Sphere(Sphere),
}

impl AsRef<Object> for Shape {
    fn as_ref(&self) -> &Object {
        match self {
            Self::Plane(p) => &p.0,
            Self::Sphere(s) => &s.0,
        }
    }
}

impl AsMut<Object> for Shape {
    fn as_mut(&mut self) -> &mut Object {
        match self {
            Self::Plane(p) => &mut p.0,
            Self::Sphere(s) => &mut s.0,
        }
    }
}

fn common_intersect<F>(object: &Object, ray: &Ray, local_intersect: F) -> Vec<f64>
where
    F: FnOnce(Ray) -> Vec<f64>,
{
    let object_ray = ray.transform(object.transform.inverse());
    local_intersect(object_ray)
}

fn common_normal_at<F>(object: &Object, point: Point, local_normal_at: F) -> Vector
where
    F: FnOnce(Point) -> Vector,
{
    let object_point = object.transform.inverse() * point;
    let object_normal = local_normal_at(object_point);
    let mut world_normal = object.transform.inverse().transpose() * object_normal;
    world_normal.0.w = 0.0;

    #[allow(clippy::unwrap_used)]
    world_normal.normalize().unwrap()
}

impl Shape {
    pub(crate) fn intersect(&self, ray: &Ray) -> Vec<Intersection<'_>> {
        let local_intersect = |object_ray| match self {
            Self::Sphere(s) => s.local_intersect(&object_ray),
            Self::Plane(p) => p.local_intersect(&object_ray),
        };

        common_intersect(self.as_ref(), ray, local_intersect)
            .into_iter()
            .map(|t| Intersection { t, object: self })
            .collect()
    }

    pub(crate) fn normal_at(&self, point: Point) -> Vector {
        let local_normal_at = |object_point| match self {
            Self::Sphere(s) => s.local_normal_at(object_point),
            Self::Plane(p) => p.local_normal_at(object_point),
        };

        common_normal_at(self.as_ref(), point, local_normal_at)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    struct TestShape {
        object: Object,
        saved_ray: Option<Ray>,
    }

    impl TestShape {
        fn intersect(&mut self, ray: &Ray) -> Vec<f64> {
            common_intersect(&self.object, ray, |object_ray| {
                self.saved_ray = Some(object_ray);
                Vec::new()
            })
        }

        fn normal_at(&self, point: Point) -> Vector {
            common_normal_at(&self.object, point, |object_point| {
                Vector::new(object_point.0.x, object_point.0.y, object_point.0.z)
            })
        }
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let o = Shape::Sphere(Sphere::default());

        let xs = o.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, &o);
        assert_eq!(xs[1].object, &o);
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut s = TestShape {
            object: Object {
                transform: Transform::try_scaling(2.0, 2.0, 2.0).unwrap(),
                ..Default::default()
            },
            saved_ray: None,
        };

        s.intersect(&r);

        assert_eq!(
            s.saved_ray,
            Some(Ray {
                origin: Point::new(0.0, 0.0, -2.5),
                direction: Vector::new(0.0, 0.0, 0.5)
            })
        );
    }

    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut s = TestShape {
            object: Object {
                transform: Transform::translation(5.0, 0.0, 0.0),
                ..Default::default()
            },
            saved_ray: None,
        };

        s.intersect(&r);

        assert_eq!(
            s.saved_ray,
            Some(Ray {
                origin: Point::new(-5.0, 0.0, -5.0),
                direction: Vector::new(0.0, 0.0, 1.0),
            })
        );
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let s = TestShape {
            object: Object {
                transform: Transform::translation(0.0, 1.0, 0.0),
                ..Default::default()
            },
            saved_ray: None,
        };

        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));

        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let o = TestShape {
            object: Object {
                transform: Transform::try_scaling(1.0, 0.5, 1.0).unwrap()
                    * Transform::rotation_z(std::f64::consts::PI / 5.0),
                ..Default::default()
            },
            saved_ray: None,
        };

        let n = o.normal_at(Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0));

        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }
}
