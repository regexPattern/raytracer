use crate::{material::Material, transform::Transform};

use super::{Bounds, Shape};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShapeProps {
    pub material: Material,
    pub transform: Transform,
    pub(crate) transform_inverse: Transform,
    pub(crate) local_bounds: Bounds,
    pub(crate) world_bounds: Bounds,
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

impl ShapeProps {
    pub fn new(material: Material, transform: Transform, local_bounds: Bounds) -> Self {
        Self {
            material,
            transform,
            transform_inverse: transform.inverse(),
            local_bounds,
            world_bounds: local_bounds.transform(transform),
        }
    }

    pub fn change_transform(&mut self, transform: Transform) {
        self.transform = transform;
        self.transform_inverse = transform.inverse();
        self.world_bounds = self.local_bounds.transform(transform);
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Point;

    use super::*;

    #[test]
    fn creating_shape_props() {
        let material = Material {
            ambient: 1.0,
            specular: 0.11,
            ..Default::default()
        };

        let transform = Transform::shearing(1.0, 2.0, 3.0, 4.0, 5.0, 6.0).unwrap();

        let local_bounds = Bounds {
            min: Point::new(1.0, 2.0, 3.0),
            max: Point::new(3.0, 2.0, 1.0),
        };

        let props = ShapeProps::new(material.clone(), transform, local_bounds);

        assert_eq!(props.material, material);
        assert_eq!(props.transform, transform);
        assert_eq!(props.local_bounds, local_bounds);
        assert_eq!(props.world_bounds, local_bounds.transform(transform));
    }

    #[test]
    fn changing_a_shape_props_transformation() {
        let local_bounds = Bounds {
            min: Point::new(1.0, 2.0, 3.0),
            max: Point::new(3.0, 2.0, 1.0),
        };

        let mut props = ShapeProps::new(
            Default::default(),
            Transform::scaling(1.0, 2.0, 3.0).unwrap(),
            local_bounds,
        );

        // Chaging a `ShaepProps`'s transform.
        let new_transform = Transform::translation(1.0, 11.0, 111.0);
        props.change_transform(new_transform);

        assert_eq!(props.transform, new_transform);
        assert_eq!(props.transform_inverse, new_transform.inverse());

        // 🔴 `local_bounds` should stay the same, but the `world_bounds` should be addapted to the
        // new props' transform.
        assert_eq!(props.local_bounds, local_bounds);
        assert_eq!(props.world_bounds, local_bounds.transform(new_transform));
    }
}
