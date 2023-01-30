use crate::{material::Material, transform::Transform};

use super::{Bounds, Shape};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShapeProps {
    pub material: Material,
    pub transform: Transform,
    pub(crate) transform_inverse: Transform,
    pub(crate) bounds: Bounds,
}

impl AsRef<ShapeProps> for Shape {
    fn as_ref(&self) -> &ShapeProps {
        match self {
            Self::Cube(inner_cube) => &inner_cube.0,
            Self::Cylinder(inner_cylinder) => &inner_cylinder.props,
            Self::Plane(inner_plane) => &inner_plane.0,
            Self::SmoothTriangle(inner_triangle) => &inner_triangle.triangle.props,
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
            Self::SmoothTriangle(inner_triangle) => &mut inner_triangle.triangle.props,
            Self::Group(inner_group) => &mut inner_group.props,
        }
    }
}
