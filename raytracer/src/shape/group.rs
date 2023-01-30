use crate::{intersection::Intersection, ray::Ray, transform::Transform};

use super::{Bounds, Shape, ShapeProps};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Group {
    pub(crate) children: Vec<Shape>,
    pub(crate) props: ShapeProps,
}

impl Group {
    pub fn new(transform: Transform) -> Self {
        Self {
            children: vec![],
            props: ShapeProps {
                transform,
                transform_inverse: transform.inverse(),
                ..Default::default()
            },
        }
    }

    pub fn push(&mut self, mut child: Shape) {
        Self::apply_transform_to_child(&mut child, self.props.transform);
        self.children.push(child);
    }

    fn apply_transform_to_child(child: &mut Shape, transform: Transform) {
        if let Shape::Group(subgroup) = child {
            for child in &mut subgroup.children {
                Self::apply_transform_to_child(child, transform);
            }
        }

        let new_transform = transform * child.as_ref().transform;

        child.as_mut().transform = new_transform;
        child.as_mut().transform_inverse = new_transform.inverse();
    }

    pub fn extend<T>(&mut self, children: T)
    where
        T: IntoIterator<Item = Shape>,
    {
        for child in children {
            self.push(child);
        }
    }

    pub(crate) fn local_intersect(&self, ray: &Ray) -> Vec<Intersection<'_>> {
        if !self.bounds().intersect(ray) {
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

    pub fn divide(&mut self, threshold: usize) {
        if threshold <= self.children.len() {
            let (left_children, right_children) = self.partition_children();

            if !left_children.is_empty() {
                self.make_subgroup(left_children);
            }

            if !right_children.is_empty() {
                self.make_subgroup(right_children);
            }
        }

        for child in &mut self.children {
            if let Shape::Group(subgroup) = child {
                subgroup.divide(threshold)
            }
        }
    }

    fn partition_children(&mut self) -> (Vec<Shape>, Vec<Shape>) {
        let (left_bounds, right_bounds) = self.bounds().split();

        let mut left_children = Vec::with_capacity(self.children.len());
        let mut right_children = Vec::with_capacity(self.children.len());

        let mut i = 0;
        while i < self.children.len() {
            let child = &mut self.children[i];
            let child_bounds = child.parent_space_bounds();

            if left_bounds.contains_box(&child_bounds) {
                child.as_mut().transform = self.props.transform_inverse * child.as_ref().transform;
                left_children.push(self.children.swap_remove(i));
            } else if right_bounds.contains_box(&child_bounds) {
                child.as_mut().transform = self.props.transform_inverse * child.as_ref().transform;
                right_children.push(self.children.swap_remove(i));
            } else {
                i += 1;
            }
        }

        (left_children, right_children)
    }

    fn make_subgroup<T>(&mut self, children: T)
    where
        T: IntoIterator<Item = Shape>,
    {
        let mut subgroup = Self::default();
        for child in children {
            subgroup.push(child);
        }

        self.push(Shape::Group(subgroup));
    }

    fn bounds(&self) -> Bounds {
        let mut bounds = Bounds::default();

        for child in &self.children {
            let child_bounds = child.parent_space_bounds();
            bounds.merge(child_bounds);
        }

        bounds
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
    fn intersecting_a_ray_with_an_empty_group() {
        let group = Group::default();

        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = group.local_intersect(&r);

        assert!(xs.is_empty());
    }

    #[test]
    fn intersecting_a_ray_with_a_non_empty_group() {
        let child0 = Shape::Sphere(Default::default());
        let child1 =
            Shape::Sphere(Sphere::default().with_transform(Transform::translation(0.0, 0.0, -3.0)));
        let child2 =
            Shape::Sphere(Sphere::default().with_transform(Transform::translation(5.0, 0.0, 0.0)));

        let mut group = Group::default();

        group.push(child0);
        group.push(child1);
        group.push(child2);

        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = group.local_intersect(&r);

        assert_eq!(xs.len(), 4);

        let child0 = &group.children[0];
        let child1 = &group.children[1];

        // Intersections are sorted by `t`.
        assert_eq!(xs[0].object, child1);
        assert_eq!(xs[1].object, child1);
        assert_eq!(xs[2].object, child0);
        assert_eq!(xs[3].object, child0);
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let child =
            Shape::Sphere(Sphere::default().with_transform(Transform::translation(5.0, 0.0, 0.0)));

        let mut group = Group::new(Transform::scaling(2.0, 2.0, 2.0).unwrap());
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

        let mut group = Group::new(Transform::scaling(2.0, 2.0, 2.0).unwrap());
        group.push(child0);
        group.push(child1);

        let bounds = group.bounds();

        assert_eq!(bounds.min, Point::new(-9.0, -6.0, -10.0));
        assert_eq!(bounds.max, Point::new(8.0, 14.0, 9.0));
    }

    #[test]
    fn partitioning_a_groups_children() {
        let s0 =
            Shape::Sphere(Sphere::default().with_transform(Transform::translation(-2.0, 0.0, 0.0)));
        let s1 =
            Shape::Sphere(Sphere::default().with_transform(Transform::translation(2.0, 0.0, 0.0)));
        let s2 = Shape::Sphere(Default::default());

        let mut group = Group::default();

        group.push(s0.clone());
        group.push(s1.clone());
        group.push(s2.clone());

        let (left, right) = group.partition_children();

        assert_eq!(group.children, vec![s2]);
        assert_eq!(left, vec![s0]);
        assert_eq!(right, vec![s1]);
    }

    #[test]
    fn creating_a_subgroup_from_a_list_of_children() {
        let s0 = Shape::Sphere(Default::default());
        let s1 = Shape::Sphere(Default::default());

        let mut group = Group::default();
        group.make_subgroup([s0.clone(), s1.clone()]);

        assert_eq!(group.children.len(), 1);

        let subgroup = match &group.children[0] {
            Shape::Group(subgroup) => subgroup,
            _ => panic!(),
        };

        assert_eq!(subgroup.children, vec![s0, s1]);
    }

    #[test]
    fn subdividing_a_group_partitions_its_children() {
        let s0 =
            Shape::Sphere(Sphere::default().with_transform(Transform::translation(-2.0, 0.0, 0.0)));
        let s1 =
            Shape::Sphere(Sphere::default().with_transform(Transform::translation(2.0, 0.0, 0.0)));
        let s2 = Shape::Sphere(
            Sphere::default().with_transform(Transform::scaling(4.0, 4.0, 4.0).unwrap()),
        );

        let mut group = Group::default();

        group.push(s0.clone());
        group.push(s1.clone());
        group.push(s2.clone());

        group.divide(1);

        assert_eq!(group.children[0], s2);

        // Children that fit into left or right bounds are added to a new subgroup.
        let subgroup = match &group.children[1] {
            Shape::Group(subgroup) => subgroup,
            _ => panic!(),
        };

        assert_eq!(subgroup.children[0], s0);
        assert_eq!(subgroup.children[1], s1);
    }
}
