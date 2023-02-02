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
mod object;
mod plane;
mod smooth_triangle;
mod sphere;
mod triangle;

pub use self::{
    cube::Cube,
    cylinder::{Cylinder, CylinderBuilder},
    group::{Group, GroupBuilder},
    plane::Plane,
    smooth_triangle::SmoothTriangle,
    sphere::Sphere,
    triangle::{Error as TriangleError, Triangle, TriangleBuilder},
};

pub(crate) use self::bounding_box::BoundingBox;

/// Available types of shapes.
#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Cube(cube::Cube),
    Cylinder(cylinder::Cylinder),
    Group(group::Group),
    Plane(plane::Plane),
    SmoothTriangle(smooth_triangle::SmoothTriangle),
    Sphere(sphere::Sphere),
    Triangle(triangle::Triangle),
}

/// Builder for a simple shape.
///
/// This includes the shapes such as: [Cube](cube::Cube), [Plane](plane::Plane) and
/// [Sphere](sphere::Sphere).
///
/// # Examples
///
/// Building a shape.
///
/// ```
/// use raytracer::{
///     material::Material,
///     shape::{Sphere, Shape, ShapeBuilder},
///     transform::Transform,
/// };
///
/// let shape = Shape::Sphere(Sphere::from(ShapeBuilder {
///     material: Material {
///         ambient: 0.5,
///         diffuse: 0.7,
///         specular: 0.1,
///         ..Default::default()
///     },
///     transform: Transform::scaling(1.0, 2.0, 3.0).unwrap(),
/// }));
/// ```
///
#[derive(Clone, Default)]
pub struct ShapeBuilder {
    /// Material of the shape.
    pub material: Material,

    /// Transform of the shape.
    pub transform: Transform,
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

    // The point is always ensured to be on the object surface so a non-null world normal always
    // exists for any object type, meaning it can always be normalized.
    #[allow(clippy::unwrap_used)]
    world_normal.normalize().unwrap()
}

impl Shape {
    pub(crate) fn intersect(&self, ray: &Ray) -> Vec<Intersection<'_>> {
        let object_ray = object_ray(ray, self.as_ref().transform_inverse);

        match self {
            Self::Cube(cube) => cube.intersect(self, &object_ray),
            Self::Cylinder(cylinder) => cylinder.intersect(self, &object_ray),
            Self::Plane(plane) => plane.intersect(self, &object_ray),
            Self::SmoothTriangle(triangle) => triangle.intersect(self, &object_ray),
            Self::Sphere(sphere) => sphere.local_intersect(self, &object_ray),
            Self::Triangle(triangle) => triangle.intersect(self, &object_ray),

            // Notice that here we pass the untransformed world ray instead of the `object` ray,
            // because a group's intersections are only the intersections of it's children, which
            // already take into account this conversion when their `Shape::intersect` method it's
            // called.
            Self::Group(group) => group.local_intersect(ray),
        }
    }

    pub(crate) fn normal_at(&self, point: Point, hit: &Intersection<'_>) -> Vector {
        world_normal(
            point,
            self.as_ref().transform_inverse,
            |object_point| match &self {
                Self::Cube(inner_cube) => inner_cube.normal_at(object_point),
                Self::Cylinder(inner_cylinder) => inner_cylinder.normal_at(object_point),
                Self::Plane(inner_plane) => inner_plane.normal_at(object_point),
                Self::SmoothTriangle(inner_triangle) => inner_triangle.normal_at(object_point, hit),
                Self::Sphere(inner_sphere) => inner_sphere.local_normal_at(object_point),
                Self::Triangle(inner_triangle) => inner_triangle.normal_at(object_point),

                // A group is never going to be asked for it's normal at certain point because the
                // normals are used to get shading information of an intersected point, however, a
                // group's intersections are only a collection of it's children intersections, so
                // the `normal_at` is called for a group's child instead that for the group itself.
                Self::Group(_) => unreachable!(),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::shape::{group::Group, sphere::Sphere};

    use super::*;

    #[test]
    fn intersecting_a_scaled_object_with_a_ray() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let transform = Transform::scaling(2.0, 2.0, 2.0).unwrap();

        assert_eq!(
            object_ray(&ray, transform.inverse()),
            Ray {
                origin: Point::new(0.0, 0.0, -2.5),
                direction: Vector::new(0.0, 0.0, 0.5)
            }
        );
    }

    #[test]
    fn intersecting_a_translated_object_with_a_ray() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let transform = Transform::translation(5.0, 0.0, 0.0);

        assert_eq!(
            object_ray(&ray, transform.inverse()),
            Ray {
                origin: Point::new(-5.0, 0.0, -5.0),
                direction: Vector::new(0.0, 0.0, 1.0),
            }
        );
    }

    #[test]
    fn computing_the_normal_on_a_translated_object() {
        let point = Point::new(0.0, 1.70711, -0.70711);
        let transform = Transform::translation(0.0, 1.0, 0.0);

        let normal = world_normal(point, transform.inverse(), |object_point| {
            Vector::new(object_point.0.x, object_point.0.y, object_point.0.z)
        });

        assert_eq!(normal, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_object() {
        let point = Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let transform = Transform::scaling(1.0, 0.5, 1.0).unwrap()
            * Transform::rotation_z(std::f64::consts::PI / 5.0);

        let normal = world_normal(point, transform.inverse(), |object_point| {
            Vector::new(object_point.0.x, object_point.0.y, object_point.0.z)
        });

        assert_eq!(normal, Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn finding_the_normal_on_a_child_object() {
        let child = Shape::Sphere(Sphere::from(ShapeBuilder {
            transform: Transform::translation(5.0, 0.0, 0.0),
            ..Default::default()
        }));

        let mut inner_group = Group::from(GroupBuilder {
            children: [],
            transform: Transform::scaling(1.0, 2.0, 3.0).unwrap(),
        });
        inner_group.push(child);

        let mut outer_group = Group::from(GroupBuilder {
            children: [],
            transform: Transform::rotation_y(std::f64::consts::FRAC_PI_2),
        });
        outer_group.push(Shape::Group(inner_group));

        // Retreiving the `inner_group`'s child. Clone would not work here since after adding the
        // child to the group (takes ownership of it), it's new parent's transformation is applied.
        let child = match &outer_group.children[0] {
            Shape::Group(inner_group) => &inner_group.children[0],
            _ => panic!(),
        };

        let n = child.normal_at(
            Point::new(1.7321, 1.1547, -5.5774),
            &Intersection {
                t: 0.0,
                object: child,
                u: None,
                v: None,
            },
        );

        // A child parent's transformations are taken into account when converting a normal in
        // it's object space to world space.
        assert_eq!(n, Vector::new(0.2857, 0.42854, -0.85716));
    }

    #[test]
    fn querying_a_shapes_bounding_box_in_its_parents_space() {
        let s = Shape::Sphere(Sphere::from(ShapeBuilder {
            transform: Transform::translation(1.0, -3.0, 5.0)
                * Transform::scaling(0.5, 2.0, 4.0).unwrap(),
            ..Default::default()
        }));

        let bounding_box = s.as_ref().parent_space_bounding_box;

        assert_eq!(bounding_box.min, Point::new(0.5, -5.0, 1.0));
        assert_eq!(bounding_box.max, Point::new(1.5, -1.0, 9.0));
    }
}
