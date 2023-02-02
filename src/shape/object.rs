use crate::{material::Material, transform::Transform};

use super::{BoundingBox, Shape};

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ObjectCache {
    pub material: Material,
    pub transform: Transform,
    pub transform_inverse: Transform,
    pub bounding_box: BoundingBox,
    pub parent_space_bounding_box: BoundingBox,
}

impl AsRef<ObjectCache> for Shape {
    fn as_ref(&self) -> &ObjectCache {
        match self {
            Self::Cube(inner_cube) => &inner_cube.0,
            Self::Cylinder(inner_cylinder) => &inner_cylinder.object_cache,
            Self::Group(inner_group) => &inner_group.object_cache,
            Self::Plane(inner_plane) => &inner_plane.0,
            Self::SmoothTriangle(inner_triangle) => &inner_triangle.triangle.object_cache,
            Self::Sphere(inner_sphere) => &inner_sphere.0,
            Self::Triangle(inner_triangle) => &inner_triangle.object_cache,
        }
    }
}

impl AsMut<ObjectCache> for Shape {
    fn as_mut(&mut self) -> &mut ObjectCache {
        match self {
            Self::Cube(inner_cube) => &mut inner_cube.0,
            Self::Cylinder(inner_cylinder) => &mut inner_cylinder.object_cache,
            Self::Group(inner_group) => &mut inner_group.object_cache,
            Self::Plane(inner_plane) => &mut inner_plane.0,
            Self::SmoothTriangle(inner_triangle) => &mut inner_triangle.triangle.object_cache,
            Self::Sphere(inner_sphere) => &mut inner_sphere.0,
            Self::Triangle(inner_triangle) => &mut inner_triangle.object_cache,
        }
    }
}

impl ObjectCache {
    pub fn new(material: Material, transform: Transform, bounding_box: BoundingBox) -> Self {
        Self {
            material,
            transform,
            transform_inverse: transform.inverse(),
            bounding_box,
            parent_space_bounding_box: bounding_box.transform(transform),
        }
    }
}
