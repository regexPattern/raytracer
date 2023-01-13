use crate::{
    intersections::Intersection,
    material::Material,
    ray::Ray,
    transform::Transform,
    tuple::{Point, Vector},
};

mod cube;
mod cylinder;
mod group;
mod plane;
mod sphere;

pub use cube::Cube;
pub use cylinder::Cylinder;
pub use group::Group;
pub use plane::Plane;
pub use sphere::Sphere;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Cube(Cube),
    Cylinder(Cylinder),
    Group(Group),
    Plane(Plane),
    Sphere(Sphere),
}

fn object_ray(ray: &Ray, transform: Transform) -> Ray {
    ray.transform(transform.inverse())
}

fn world_normal<F>(point: Point, transform: Transform, local_normal_at: F) -> Vector
where
    F: Fn(Point) -> Vector,
{
    let object_point = transform.inverse() * point;
    let object_normal = local_normal_at(object_point);
    let mut world_normal = transform.inverse().transpose() * object_normal;
    world_normal.0.w = 0.0;

    // The point is ensured to always be on the object surface so a non-null normal always exists
    // for any object type.
    #[allow(clippy::unwrap_used)]
    world_normal.normalize().unwrap()
}

impl Object {
    pub fn intersect(&self, world_ray: &Ray) -> Vec<Intersection<'_>> {
        let object_ray = object_ray(world_ray, self.transform());

        match self {
            Self::Cube(_) => Cube::local_intersect(self, object_ray),
            Self::Cylinder(cylinder) => cylinder.local_intersect(self, object_ray),
            Self::Group(group) => group.local_intersect(&world_ray),
            Self::Plane(_) => Plane::local_intersect(self, object_ray),
            Self::Sphere(_) => Sphere::local_intersect(self, object_ray),
        }
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        world_normal(point, self.transform(), |object_point| {
            match &self {
                Self::Cube(_) => Cube::local_normal_at(object_point),
                Self::Cylinder(cylinder) => cylinder.local_normal_at(object_point),
                Self::Plane(_) => Plane::local_normal_at(object_point),
                Self::Sphere(_) => Sphere::local_normal_at(object_point),

                // This function is never called, since an object's normal is used only when shading
                // this object, accessed through the vector of intersections that `Object::intersect`
                // returns. In the case of a `Group`, these intersections never have another `Group`
                // inside because of the recursive implementation and flattening happening in
                // `group::local_intersect`.
                Self::Group(_) => unreachable!(),
            }
        })
    }

    fn world_to_object(&self, point: Point) -> Point {
        self.transform().inverse() * point
    }

    fn normal_to_world(&self, normal: Vector) -> Vector {
        let mut normal = self.transform().inverse().transpose() * normal;
        normal.0.w = 0.0;

        // The point is ensured to always be on the object surface so a non-null normal always exists
        // for any object type.
        #[allow(clippy::unwrap_used)]
        normal.normalize().unwrap()
    }

    pub fn material(&self) -> &Material {
        match self {
            Self::Cube(cube) => &cube.material,
            Self::Cylinder(cylinder) => &cylinder.material,
            Self::Plane(plane) => &plane.material,
            Self::Sphere(sphere) => &sphere.material,

            // Same reason as `Self::normal_at`.
            Self::Group(_) => unreachable!(),
        }
    }

    pub fn material_mut(&mut self) -> &mut Material {
        match self {
            Self::Cube(cube) => &mut cube.material,
            Self::Cylinder(cylinder) => &mut cylinder.material,
            Self::Plane(plane) => &mut plane.material,
            Self::Sphere(sphere) => &mut sphere.material,

            // Same reason as `Self::normal_at`.
            Self::Group(_) => unreachable!(),
        }
    }

    // TODO: Maybe I could build a derive macro that implements the mutable version of this match.
    pub fn transform(&self) -> Transform {
        match self {
            Self::Cube(cube) => cube.transform,
            Self::Cylinder(cylinder) => cylinder.transform,
            Self::Plane(plane) => plane.transform,
            Self::Sphere(sphere) => sphere.transform,
            Self::Group(group) => group.transform,
        }
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        match self {
            Self::Cube(cube) => &mut cube.transform,
            Self::Cylinder(cylinder) => &mut cylinder.transform,
            Self::Plane(plane) => &mut plane.transform,
            Self::Sphere(sphere) => &mut sphere.transform,
            Self::Group(group) => &mut group.transform,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestObject {
        transform: Transform,
        saved_ray: Option<Ray>,
    }

    impl TestObject {
        fn intersect(&mut self, ray: &Ray) -> Vec<f64> {
            self.saved_ray = Some(object_ray(ray, self.transform));
            vec![]
        }

        fn normal_at(&self, point: Point) -> Vector {
            world_normal(point, self.transform, |object_point| {
                Vector::new(object_point.0.x, object_point.0.y, object_point.0.z)
            })
        }
    }

    fn get_subgroup_child(super_group: &Group) -> &Object {
        match &super_group.children[0] {
            Object::Group(sub_group) => &sub_group.children[0],
            _ => unimplemented!(),
        }
    }

    #[test]
    fn intersecting_a_scaled_object_with_a_ray() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut s = TestObject {
            transform: Transform::try_scaling(2.0, 2.0, 2.0).unwrap(),
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
    fn intersecting_a_translated_object_with_a_ray() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut s = TestObject {
            transform: Transform::translation(5.0, 0.0, 0.0),
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
    fn computing_the_normal_on_a_translated_object() {
        let o = TestObject {
            transform: Transform::translation(0.0, 1.0, 0.0),
            saved_ray: None,
        };

        let n = o.normal_at(Point::new(0.0, 1.70711, -0.70711));

        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_object() {
        let o = TestObject {
            transform: Transform::try_scaling(1.0, 0.5, 1.0).unwrap()
                * Transform::rotation_z(std::f64::consts::PI / 5.0),
            saved_ray: None,
        };

        let n = o.normal_at(Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0));

        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn converting_a_point_from_world_to_object_space() {
        let s = Object::Sphere(Sphere {
            transform: Transform::translation(5.0, 0.0, 0.0),
            ..Default::default()
        });

        let mut g2 = Group {
            transform: Transform::try_scaling(2.0, 2.0, 2.0).unwrap(),
            ..Default::default()
        };

        g2.add_child(s);

        let mut g1 = Group {
            transform: Transform::rotation_y(std::f64::consts::FRAC_PI_2),
            ..Default::default()
        };

        g1.add_child(Object::Group(g2));

        let s = get_subgroup_child(&g1);

        let p = s.world_to_object(Point::new(-2.0, 0.0, -10.0));

        assert_eq!(p, Point::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn converting_a_normal_from_object_to_world_space() {
        let s = Object::Sphere(Sphere {
            transform: Transform::translation(5.0, 0.0, 0.0),
            ..Default::default()
        });

        let mut g2 = Group {
            transform: Transform::try_scaling(1.0, 2.0, 3.0).unwrap(),
            ..Default::default()
        };

        g2.add_child(s);

        let mut g1 = Group {
            transform: Transform::rotation_y(std::f64::consts::FRAC_PI_2),
            ..Default::default()
        };

        g1.add_child(Object::Group(g2));

        let s = get_subgroup_child(&g1);

        let n = s.normal_to_world(Vector::new(
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
        ));

        assert_eq!(n, Vector::new(0.28571, 0.42857, -0.85714));
    }

    #[test]
    fn finding_the_normal_on_a_child_object() {
        let s = Object::Sphere(Sphere {
            transform: Transform::translation(5.0, 0.0, 0.0),
            ..Default::default()
        });

        let mut g2 = Group {
            transform: Transform::try_scaling(1.0, 2.0, 3.0).unwrap(),
            ..Default::default()
        };

        g2.add_child(s);

        let mut g1 = Group {
            transform: Transform::rotation_y(std::f64::consts::FRAC_PI_2),
            ..Default::default()
        };

        g1.add_child(Object::Group(g2));

        let s = get_subgroup_child(&g1);

        let n = s.normal_at(Point::new(1.7321, 1.1547, -5.5774));

        assert_eq!(n, Vector::new(0.2857, 0.42854, -0.85716));
    }
}
