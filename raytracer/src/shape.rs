use crate::{
    intersection::Intersection,
    material::Material,
    ray::Ray,
    transform::Transform,
    tuple::{Point, Vector},
};

mod bounding_box;
mod cube;
mod cylinder;
mod group;
mod plane;
mod sphere;
mod triangle;

pub use bounding_box::BoundingBox;
pub use cylinder::Cylinder;
pub use group::Group;
pub use triangle::{CollinearTriangleSidesError, Triangle};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BaseShape {
    pub material: Material,
    pub transform: Transform,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Cube(BaseShape),
    Cylinder(Cylinder),
    Group(Group),
    Plane(BaseShape),
    Sphere(BaseShape),
    Triangle(Triangle),
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

impl Shape {
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection<'_>> {
        let object_ray = object_ray(ray, self.get_transform());

        match self {
            Self::Cube(_) => cube::intersect(self, &object_ray, &cube::bounding_box()),
            Self::Cylinder(cylinder) => cylinder.intersect(self, &object_ray),
            Self::Group(group) => group.intersect(&ray),
            Self::Plane(_) => plane::intersect(self, &object_ray),
            Self::Sphere(_) => sphere::intersect(self, &object_ray),
            Self::Triangle(triangle) => triangle.intersect(self, &object_ray),
        }
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        world_normal(point, self.get_transform(), |object_point| {
            match &self {
                Self::Cube(_) => cube::normal_at(object_point),
                Self::Cylinder(cylinder) => cylinder.normal_at(object_point),
                Self::Plane(_) => plane::normal_at(object_point),
                Self::Sphere(_) => sphere::normal_at(object_point),
                Self::Triangle(triangle) => triangle.normal_at(object_point),

                // This function is never called, since an object's normal is used only when shading
                // this object, accessed through the vector of intersections that `Object::intersect`
                // returns. In the case of a `Group`, these intersections never have another `Group`
                // inside because of the recursive implementation and flattening happening in
                // `group::local_intersect`.
                Self::Group(_) => unreachable!(),
            }
        })
    }

    pub fn divide(&mut self, threshold: usize) {
        if let Shape::Group(group) = self {
            group.divide(threshold);
        }
    }

    pub fn get_bounding_box(&self) -> BoundingBox {
        let bbox = match self {
            Self::Cube(_) => cube::bounding_box(),
            Self::Cylinder(cylinder) => cylinder.bounding_box(),
            Self::Plane(_) => plane::bounding_box(),
            Self::Sphere(_) => sphere::bounding_box(),
            Self::Triangle(triangle) => triangle.bounding_box(),
            Self::Group(group) => group.bounding_box(),
        };

        bbox.transform(self.get_transform())
    }

    pub fn get_material(&self) -> &Material {
        match self {
            Self::Cube(bs) | Self::Plane(bs) | Self::Sphere(bs) => &bs.material,
            Self::Cylinder(cylinder) => &cylinder.base_shape.material,
            Self::Triangle(triangle) => &triangle.material,
            _ => unimplemented!(),
        }
    }

    pub fn set_material(&mut self, material: Material) {
        match self {
            Self::Cube(bs) | Self::Plane(bs) | Self::Sphere(bs) => bs.material = material,
            Self::Cylinder(cylinder) => cylinder.base_shape.material = material,
            _ => (),
        }
    }

    pub fn get_transform(&self) -> Transform {
        match self {
            Self::Cube(bs) | Self::Plane(bs) | Self::Sphere(bs) => bs.transform,
            Self::Cylinder(cylinder) => cylinder.base_shape.transform,
            Self::Triangle(_) => Default::default(),
            Self::Group(group) => group.transform,
        }
    }

    pub fn set_transform(&mut self, transform: Transform) {
        match self {
            Self::Cube(bs) | Self::Plane(bs) | Self::Sphere(bs) => bs.transform = transform,
            Self::Cylinder(cylinder) => cylinder.base_shape.transform = transform,
            Self::Triangle(_) => (),
            Self::Group(group) => group.update_transform(transform),
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

    fn get_subgroup_child(super_group: &Group) -> &Shape {
        match &super_group.children[0] {
            Shape::Group(sub_group) => &sub_group.children[0],
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
    fn finding_the_normal_on_a_child_object() {
        let s = Shape::Sphere(BaseShape {
            transform: Transform::translation(5.0, 0.0, 0.0),
            ..Default::default()
        });

        let g2 = Group::new([s], Transform::try_scaling(1.0, 2.0, 3.0).unwrap());

        let g1 = Group::new(
            [Shape::Group(g2)],
            Transform::rotation_y(std::f64::consts::FRAC_PI_2),
        );

        let s = get_subgroup_child(&g1);

        let n = s.normal_at(Point::new(1.7321, 1.1547, -5.5774));

        assert_eq!(n, Vector::new(0.2857, 0.42854, -0.85716));
    }

    #[test]
    fn querying_a_shapess_bounding_box_in_its_parent_s_space() {
        let s = Shape::Sphere(BaseShape {
            transform: Transform::translation(1.0, -3.0, 5.0)
                * Transform::try_scaling(0.5, 2.0, 4.0).unwrap(),
            ..Default::default()
        });

        let bbox = s.get_bounding_box();

        assert_eq!(bbox.min, Point::new(0.5, -5.0, 1.0));
        assert_eq!(bbox.max, Point::new(1.5, -1.0, 9.0));
    }

    #[test]
    fn subdividing_a_primitive_does_nothing() {
        let mut s = Shape::Sphere(Default::default());
        s.divide(1);

        assert_eq!(s, Shape::Sphere(Default::default()));
    }
}
