use crate::{
    intersection::Intersection,
    material::Material,
    ray::Ray,
    transform::Transform,
    tuple::{Point, Vector},
};

mod bounds;
mod cube;
mod cylinder;
mod group;
mod plane;
mod sphere;
mod triangle;

pub use self::bounds::Bounds;
pub use self::cube::Cube;
pub use self::cylinder::Cylinder;
pub use self::group::Group;
pub use self::plane::Plane;
pub use self::sphere::Sphere;
pub use self::triangle::{CollinearTriangleSidesError, Triangle};

#[derive(Clone, Debug, PartialEq)]
pub struct ShapeProps {
    pub material: Material,
    pub transform: Transform,
    pub(crate) transform_inverse: Transform,
    pub(crate) local_bounds: Bounds,
    pub(crate) world_bounds: Bounds,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Cube(Cube),
    Cylinder(Cylinder),
    Group(Group),
    Plane(Plane),
    Sphere(Sphere),
    Triangle(Triangle),
}

impl AsRef<ShapeProps> for Shape {
    fn as_ref(&self) -> &ShapeProps {
        match self {
            Self::Cube(inner_cube) => &inner_cube.0,
            Self::Cylinder(inner_cylinder) => &inner_cylinder.props,
            Self::Plane(inner_plane) => &inner_plane.0,
            Self::Sphere(inner_sphere) => &inner_sphere.0,
            Self::Triangle(inner_triangle) => &inner_triangle.props,
            Self::Group(inner_group) => &inner_group.props,
        }
    }
}

impl AsMut<ShapeProps> for Shape {
    fn as_mut(&mut self) -> &mut ShapeProps {
        match self {
            Self::Cube(inner_cube) => &mut inner_cube.0,
            Self::Cylinder(inner_cylinder) => &mut inner_cylinder.props,
            Self::Plane(inner_plane) => &mut inner_plane.0,
            Self::Sphere(inner_sphere) => &mut inner_sphere.0,
            Self::Triangle(inner_triangle) => &mut inner_triangle.props,
            Self::Group(inner_group) => &mut inner_group.props,
        }
    }
}

impl Shape {
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection<'_>> {
        let object_ray = object_ray(ray, self.as_ref().transform_inverse);

        match self {
            Self::Cube(inner_cube) => inner_cube.intersect(self, &object_ray),
            Self::Cylinder(inner_cylinder) => inner_cylinder.intersect(self, &object_ray),
            Self::Group(inner_group) => inner_group.intersect(ray),
            Self::Plane(inner_plane) => inner_plane.intersect(self, &object_ray),
            Self::Sphere(inner_sphere) => inner_sphere.intersect(self, &object_ray),
            Self::Triangle(inner_triangle) => inner_triangle.intersect(self, &object_ray),
        }
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        world_normal(
            point,
            self.as_ref().transform_inverse,
            |object_point| match &self {
                Self::Cube(inner_cube) => inner_cube.normal_at(object_point),
                Self::Cylinder(inner_cylinder) => inner_cylinder.normal_at(object_point),
                Self::Plane(inner_plane) => inner_plane.normal_at(object_point),
                Self::Sphere(inner_sphere) => inner_sphere.normal_at(object_point),
                Self::Triangle(inner_triangle) => inner_triangle.normal_at(object_point),
                Self::Group(_) => unreachable!(),
            },
        )
    }
}

fn object_ray(ray: &Ray, transform_inverse: Transform) -> Ray {
    ray.transform(transform_inverse)
}

fn world_normal<F>(point: Point, transform_inverse: Transform, local_normal_at: F) -> Vector
where
    F: Fn(Point) -> Vector,
{
    let object_point = transform_inverse * point;
    let object_normal = local_normal_at(object_point);
    let mut world_normal = transform_inverse.transpose() * object_normal;
    world_normal.0.w = 0.0;

    // The point is ensured to always be on the object surface so a non-null normal always exists
    // for any object type.
    #[allow(clippy::unwrap_used)]
    world_normal.normalize().unwrap()
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
            self.saved_ray = Some(object_ray(ray, self.transform.inverse()));
            vec![]
        }

        fn normal_at(&self, point: Point) -> Vector {
            world_normal(point, self.transform.inverse(), |object_point| {
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
        let s = Shape::Sphere(Sphere::new(
            Default::default(),
            Transform::translation(5.0, 0.0, 0.0),
        ));

        let g1 = Group::new([s], Transform::try_scaling(1.0, 2.0, 3.0).unwrap());

        let g0 = Group::new(
            [Shape::Group(g1)],
            Transform::rotation_y(std::f64::consts::FRAC_PI_2),
        );

        let s = get_subgroup_child(&g0);

        let n = s.normal_at(Point::new(1.7321, 1.1547, -5.5774));

        assert_eq!(n, Vector::new(0.2857, 0.42854, -0.85716));
    }

    #[test]
    fn querying_a_shapess_bounding_box_in_its_parents_space() {
        let s = Shape::Sphere(Sphere::new(
            Default::default(),
            Transform::translation(1.0, -3.0, 5.0) * Transform::try_scaling(0.5, 2.0, 4.0).unwrap(),
        ));

        let bounds = s.as_ref().world_bounds;

        assert_eq!(bounds.min, Point::new(0.5, -5.0, 1.0));
        assert_eq!(bounds.max, Point::new(1.5, -1.0, 9.0));
    }
}
