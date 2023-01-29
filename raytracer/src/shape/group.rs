use crate::{intersection::Intersection, ray::Ray, transform::Transform};

use super::{Bounds, Shape, ShapeProps};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Group {
    pub(crate) children: Vec<Shape>,
    pub(crate) props: ShapeProps,
}

impl Group {
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.props.change_transform(transform);
        self
    }

    pub fn push(&mut self, mut child: Shape) {
        Self::apply_transform_to_child(&mut child, self.props.transform);

        self.props.local_bounds.merge(child.as_ref().world_bounds);
        self.props.world_bounds = self.props.local_bounds;

        self.children.push(child);
    }

    pub fn change_transform(&mut self, transform: Transform) {
        let new_transform = transform * self.props.transform_inverse;

        for child in &mut self.children {
            Self::apply_transform_to_child(child, new_transform);
        }

        let mut local_bounds = Bounds::default();
        for child in &self.children {
            local_bounds.merge(child.as_ref().world_bounds);
        }

        self.props.local_bounds = local_bounds;
        self.props.transform = transform;
        self.props.transform_inverse = transform.inverse();
        self.props.world_bounds = local_bounds;
    }

    pub(crate) fn local_intersect(&self, ray: &Ray) -> Vec<Intersection<'_>> {
        if !self.props.world_bounds.intersect(ray) {
            return vec![];
        }

        let mut intersections: Vec<_> = self
            .children
            .iter()
            .flat_map(|child| child.intersect(ray))
            .collect();

        Intersection::sort(&mut intersections);
        intersections
    }

    fn apply_transform_to_child(child: &mut Shape, transform: Transform) {
        if let Shape::Group(inner_group) = child {
            for child in &mut inner_group.children {
                Self::apply_transform_to_child(child, transform);
            }
        }

        let prev_transform = child.as_ref().transform;
        child.as_mut().change_transform(transform * prev_transform);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        shape::{Cylinder, Sphere},
        transform::Transform,
        tuple::{Point, Vector},
    };

    use super::*;

    #[test]
    fn adding_a_child_to_a_group_applies_its_transform_to_the_added_children() {
        // Child
        let child_transform = Transform::shearing(1.0, 3.0, 5.0, 7.0, 9.0, 11.0).unwrap();
        let child = Shape::Sphere(Sphere::default().with_transform(child_transform));

        // Inner group
        let inner_group_transform = Transform::rotation_z(std::f64::consts::FRAC_PI_2);
        let mut inner_group = Group::default().with_transform(inner_group_transform);

        inner_group.push(child);

        // Outer group
        let outer_group_transform = Transform::translation(1.0, 2.0, 3.0);
        let mut outer_group = Group::default().with_transform(outer_group_transform);

        outer_group.push(Shape::Group(inner_group));

        // 🔴 Check if the outer group's transform is applied to the inner group.
        let inner_group = match &outer_group.children[0] {
            Shape::Group(inner_group) => inner_group,
            _ => panic!(),
        };

        assert_eq!(
            inner_group.props.transform,
            outer_group_transform * inner_group_transform
        );
        assert_eq!(
            inner_group.props.transform_inverse,
            (outer_group_transform * inner_group_transform).inverse()
        );

        // 🔴 Check if the outer and inner group's transform is applied to the leaf child node.
        let child = &inner_group.children[0];

        assert_eq!(
            child.as_ref().transform,
            inner_group.props.transform * child_transform
        );
        assert_eq!(
            child.as_ref().transform_inverse,
            (inner_group.props.transform * child_transform).inverse()
        );
    }

    #[test]
    fn updating_a_groups_transform_also_updates_its_children_transforms() {
        // Child
        let child_transform = Transform::scaling(1.0, 2.0, 3.0).unwrap();
        let child = Shape::Sphere(Sphere::default().with_transform(child_transform));

        // Inner group
        let inner_group_transform = Transform::translation(1.0, 2.0, 3.0);
        let mut inner_group = Group::default().with_transform(inner_group_transform);
        inner_group.push(child);

        // Outer group
        let outer_group_transform = Transform::rotation_x(std::f64::consts::FRAC_PI_2);
        let mut outer_group = Group::default().with_transform(outer_group_transform);
        outer_group.push(Shape::Group(inner_group));

        // Update outer_group's transform
        let new_transform = Transform::shearing(1.0, 2.0, 3.0, 4.0, 5.0, 6.0).unwrap();
        outer_group.change_transform(new_transform);

        // 🔴 Check if changes were applied to the outer group itself.
        assert_eq!(outer_group.props.transform, new_transform);

        // 🔴 Check if changes were applied to the inner group.
        let inner_group = match &outer_group.children[0] {
            Shape::Group(inner_group) => inner_group,
            _ => panic!(),
        };

        // Notice how its parent's previous transformation `outer_group_transform` is no longer
        // applied to this child, instead, this child's original transform is now being composed
        // with its parent's new transfrom `new_transform`.
        assert_eq!(
            inner_group.props.transform,
            new_transform * inner_group_transform
        );

        // 🔴 Check if changes were applied to the leaf child node.
        let child = &inner_group.children[0];

        // Same as with the outer gruop's first child, here the previous outer group's
        // transformation is no longer taken into account.
        assert_eq!(
            child.as_ref().transform,
            new_transform * inner_group_transform * child_transform
        );
    }

    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let group = Group::default();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = group.local_intersect(&ray);

        assert!(xs.is_empty());
    }

    #[test]
    fn intersecting_a_ray_with_a_non_empty_group() {
        let mut group = Group::default();

        let child0 = Shape::Sphere(Default::default());
        let child1 = Shape::Sphere(Sphere::new(
            Default::default(),
            Transform::translation(0.0, 0.0, -3.0),
        ));
        let child2 = Shape::Sphere(Sphere::new(
            Default::default(),
            Transform::translation(5.0, 0.0, 0.0),
        ));

        group.push(child0);
        group.push(child1);
        group.push(child2);

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = group.local_intersect(&ray);

        assert_eq!(xs.len(), 4);

        let child0 = &group.children[0];
        let child1 = &group.children[1];

        // 🔴 Intersections are sorted by `t`.
        assert_eq!(xs[0].object, child1);
        assert_eq!(xs[1].object, child1);
        assert_eq!(xs[2].object, child0);
        assert_eq!(xs[3].object, child0);
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let mut group = Group::default().with_transform(Transform::scaling(2.0, 2.0, 2.0).unwrap());

        let child = Shape::Sphere(Sphere::new(
            Default::default(),
            Transform::translation(5.0, 0.0, 0.0),
        ));

        group.push(child);

        let ray = Ray {
            origin: Point::new(10.0, 0.0, -10.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let group = Shape::Group(group);
        let xs = group.intersect(&ray); // Now using `intersect` instead of `local_intersect`.

        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn a_group_has_a_bouding_box_that_contains_its_children() {
        let child0 = Shape::Sphere(Sphere::default().with_transform(
            Transform::translation(2.0, 5.0, -3.0) * Transform::scaling(2.0, 2.0, 2.0).unwrap(),
        ));

        let child1 = Shape::Cylinder(Cylinder::new(
            Default::default(),
            Transform::translation(-4.0, -1.0, 4.0) * Transform::scaling(0.5, 1.0, 0.5).unwrap(),
            -2.0,
            2.0,
            false,
        ));

        let mut group = Group::default().with_transform(Transform::scaling(2.0, 2.0, 2.0).unwrap());
        group.push(child0);
        group.push(child1);

        let bounds = group.props.world_bounds;

        assert_eq!(bounds.min, Point::new(-9.0, -6.0, -10.0));
        assert_eq!(bounds.max, Point::new(8.0, 14.0, 9.0));
    }

    #[test]
    fn updating_a_groups_transform_also_updates_its_bounds() {
        let child = Shape::Sphere(
            Sphere::default().with_transform(Transform::scaling(2.0, 2.0, 2.0).unwrap()),
        );

        let mut group = Group::default().with_transform(Transform::scaling(2.0, 2.0, 2.0).unwrap());
        group.push(child);

        group.change_transform(Transform::scaling(0.5, 0.5, 0.5).unwrap());

        let bounds = group.props.world_bounds;

        assert_eq!(bounds.min, Point::new(-1.0, -1.0, -1.0));
        assert_eq!(bounds.max, Point::new(1.0, 1.0, 1.0));
    }
}
