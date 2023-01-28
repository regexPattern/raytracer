use crate::{intersection::Intersection, ray::Ray, transform::Transform};

use super::{Shape, ShapeProps};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Group {
    pub(crate) children: Vec<Shape>,
    pub(crate) props: ShapeProps,
}

impl Group {
    pub fn push(&mut self, mut child: Shape) {
        Self::apply_transform_to_child(&mut child, self.props.transform);
        self.children.push(child);
    }

    pub fn change_transform(&mut self, transform: Transform) {
        // Updating the transform implies applying the new transform to the group's children, but
        // also means that the previously applied transform has to be reversed.
        let new_transform = transform * self.props.transform.inverse();

        for child in &mut self.children {
            Self::apply_transform_to_child(child, new_transform);
        }

        self.props.change_transform(transform);
    }

    fn apply_transform_to_child(child: &mut Shape, transform: Transform) {
        // Traverse the child tree recursively if this node has children.
        if let Shape::Group(inner_group) = child {
            for inner_child in &mut inner_group.children {
                Self::apply_transform_to_child(inner_child, transform);
            }
        }

        // If this node has no more children or no children at all, then apply the transformation
        // to the node itself.
        let new_transform = transform * child.as_ref().transform;
        child.as_mut().change_transform(new_transform);
    }

    pub fn local_intersect<'a>(&self, ray: &Ray) -> Vec<Intersection<'_>> {
        let mut xs: Vec<_> = self
            .children
            .iter()
            .flat_map(|child| child.intersect(ray))
            .collect();

        Intersection::sort(&mut xs);
        xs
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        shape::Sphere,
        transform::Transform,
        tuple::{Point, Vector},
    };

    use super::*;

    // TODO: I should get rid of this function when I update the constructor for shapes.
    fn test_group_with_transform(transform: Transform) -> Group {
        Group {
            children: vec![],
            props: ShapeProps::new(Default::default(), transform, Default::default()),
        }
    }

    #[test]
    fn adding_a_child_to_a_group() {
        let child = Shape::Sphere(Default::default());

        let mut g = Group::default();
        g.push(child.clone());

        assert!(!g.children.is_empty());
        assert!(g.children.contains(&child))
    }

    #[test]
    fn adding_a_child_to_a_transformed_group() {
        let child_transform = Transform::rotation_x(std::f64::consts::FRAC_PI_2);
        let child = Shape::Sphere(Sphere::new(Default::default(), child_transform));

        let group_transform = Transform::scaling(2.0, 3.0, 1.0).unwrap();
        let mut g = test_group_with_transform(group_transform);
        g.push(child);

        let child = &g.children[0];
        let expected_transform = group_transform * child_transform;

        assert_eq!(child.as_ref().transform, expected_transform);
        assert_eq!(
            child.as_ref().transform_inverse,
            expected_transform.inverse()
        );
        assert_eq!(
            child.as_ref().world_bounds,
            child.as_ref().local_bounds.transform(expected_transform)
        );
    }

    #[test]
    fn adding_an_inner_group_to_a_group() {
        let child_transform = Transform::scaling(0.1, 0.72, 1.0).unwrap();
        let child = Shape::Sphere(Sphere::new(Default::default(), child_transform));

        let inner_group_transform = Transform::rotation_y(std::f64::consts::FRAC_PI_8);
        let mut inner_group = test_group_with_transform(inner_group_transform);
        inner_group.push(child);

        let outer_group_transform = Transform::shearing(1.0, 2.0, 3.0, 4.0, 5.0, 6.0).unwrap();
        let mut outer_group = test_group_with_transform(outer_group_transform);
        outer_group.push(Shape::Group(inner_group));

        // 🔴 The `inner_group` has been inserted as a regular child of `outer_group`, meaning it's
        // parent's transform has been applied.
        let inner_group = match &outer_group.children[0] {
            Shape::Group(inner_group) => inner_group,
            _ => panic!(),
        };

        assert_eq!(outer_group.props.transform, outer_group_transform);
        assert_eq!(
            inner_group.props.transform,
            outer_group_transform * inner_group_transform
        );

        // 🔴 The `child` has been inserted in both `inner_group` first, and `outer_group` later,
        // an both of it's parent transforms have been applied in order.
        let child = &inner_group.children[0];

        assert_eq!(
            child.as_ref().transform,
            outer_group_transform * inner_group_transform * child_transform
        );
    }

    #[test]
    fn changing_a_group_transform_changes_its_children_transforms() {
        let original_child_transform = Transform::rotation_x(std::f64::consts::FRAC_PI_2);
        let original_group_transform = Transform::scaling(2.0, 3.0, 1.0).unwrap();

        let child = Shape::Sphere(Sphere::new(Default::default(), original_child_transform));
        let mut group = test_group_with_transform(original_group_transform);

        // The child is added before changing the group's transform, so at this stage the
        // `original_group_transform` is applied to the child, on top of the child's own transform.
        group.push(child);

        let new_group_transform = Transform::shearing(1.0, 2.0, 3.0, 4.0, 5.0, 5.0).unwrap();
        group.change_transform(new_group_transform);

        // 🔴 The group's own transformation and bounds should have changed.
        assert_eq!(group.props.transform, new_group_transform);
        assert_eq!(group.props.transform_inverse, new_group_transform.inverse());
        assert_eq!(
            group.props.world_bounds,
            group.props.local_bounds.transform(new_group_transform)
        );

        // 🔴 The group's children transformations and bounds should have changed. Note that this
        // new transformation is overrides the group's previous transformation, so it should be
        // applied to the child's `original_child_transform`.
        let child = &group.children[0];
        let expected_transform = new_group_transform * original_child_transform;

        assert_eq!(child.as_ref().transform, expected_transform);
        assert_eq!(
            child.as_ref().transform_inverse,
            expected_transform.inverse()
        );
        assert_eq!(
            child.as_ref().world_bounds,
            child.as_ref().local_bounds.transform(expected_transform)
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
        let _child2 = &group.children[2];

        // 🔴 Intersections are sorted by `t`.
        assert_eq!(xs[0].object, child1);
        assert_eq!(xs[1].object, child1);
        assert_eq!(xs[2].object, child0);
        assert_eq!(xs[3].object, child0);
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let mut group = test_group_with_transform(Transform::scaling(2.0, 2.0, 2.0).unwrap());

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
}
