use crate::{intersection::Intersection, ray::Ray, transform::Transform};

use super::{Bounds, Shape, ShapeProps};

#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    pub(crate) children: Vec<Shape>,
    pub(crate) props: ShapeProps,
}

impl Default for Group {
    fn default() -> Self {
        Self::new([], Default::default())
    }
}

impl Group {
    pub fn new<T>(children: T, transform: Transform) -> Self
    where
        T: Into<Vec<Shape>>,
    {
        let mut children = children.into();
        for child in &mut children {
            transform_recursive(child, transform);
        }

        let mut local_bounds = Bounds::default();
        for child in &children {
            let child_bounds = child.as_ref().world_bounds;
            local_bounds.merge(child_bounds);
        }

        Self {
            children,
            props: ShapeProps {
                material: Default::default(),
                transform,
                transform_inverse: transform.inverse(),
                local_bounds,
                world_bounds: local_bounds.transform(transform),
            },
        }
    }

    pub(crate) fn intersect(&self, ray: &Ray) -> Vec<Intersection<'_>> {
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

    fn adjust_bounds(&mut self) {
        let mut local_bounds = Bounds::default();
        for child in &self.children {
            let child_bounds = child.as_ref().world_bounds;
            local_bounds.merge(child_bounds);
        }

        self.props.local_bounds = local_bounds;
        self.props.world_bounds = local_bounds.transform(self.props.transform);
    }

    pub fn push(&mut self, mut child: Shape) {
        transform_recursive(&mut child, self.props.transform);
        self.children.push(child);
        self.adjust_bounds();
    }

    pub fn extend<T>(&mut self, children: T)
    where
        T: IntoIterator<Item = Shape>,
    {
        // This doesn't call `push()` internally because the bounds doesn't need to be recomputed
        // after pushing each individual child. It's more efficient to adjust them after all new
        // childs have been added.
        for mut child in children {
            transform_recursive(&mut child, self.props.transform);
            self.children.push(child);
        }
        self.adjust_bounds();
    }

    pub fn partition_children(&mut self) -> (Vec<Shape>, Vec<Shape>) {
        let (left_bounds, right_bounds) = self.props.local_bounds.split();

        let mut left_children = vec![];
        let mut right_children = vec![];

        let mut i = 0;
        while i < self.children.len() {
            let child_bounds = self.children[i].as_ref().world_bounds;

            if left_bounds.contains_box(&child_bounds) {
                left_children.push(self.children.swap_remove(i));
            } else if right_bounds.contains_box(&child_bounds) {
                right_children.push(self.children.swap_remove(i));
            } else {
                i += 1;
            }
        }

        self.adjust_bounds();
        (left_children, right_children)
    }

    pub fn divide(&mut self, threshold: usize) {
        if threshold <= self.children.len() {
            let (left, right) = self.partition_children();

            if !left.is_empty() {
                self.push(Shape::Group(Group::new(left, Default::default())));
            }

            if !right.is_empty() {
                self.push(Shape::Group(Group::new(right, Default::default())));
            }
        }

        for child in &mut self.children {
            if let Shape::Group(inner_group) = child {
                inner_group.divide(threshold)
            }
        }
    }
}

fn transform_recursive(object: &mut Shape, transform: Transform) {
    if let Shape::Group(group) = object {
        for child in &mut group.children {
            transform_recursive(child, transform);
        }
    } else {
        let new_transform = transform * object.as_ref().transform;
        object.as_mut().transform = new_transform;
        object.as_mut().transform_inverse = new_transform.inverse();
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        shape::{Cylinder, Sphere},
        tuple::{Point, Vector},
    };

    use super::*;

    fn get_subgroup_child(super_group: &Group) -> &Shape {
        match &super_group.children[0] {
            Shape::Group(subgroup) => &subgroup.children[0],
            _ => unimplemented!(),
        }
    }

    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let g = Group::default();

        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = g.intersect(&r);

        assert!(xs.is_empty());
    }

    #[test]
    fn intersecting_a_ray_with_a_non_empty_group() {
        let s0 = Shape::Sphere(Default::default());

        let s1 = Shape::Sphere(Sphere::new(
            Default::default(),
            Transform::translation(0.0, 0.0, -3.0),
        ));

        let s2 = Shape::Sphere(Sphere::new(
            Default::default(),
            Transform::translation(5.0, 0.0, 0.0),
        ));

        let g = Group::new([s0.clone(), s1.clone(), s2.clone()], Default::default());

        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = g.intersect(&r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object, &s1);
        assert_eq!(xs[1].object, &s1);
        assert_eq!(xs[2].object, &s0);
        assert_eq!(xs[3].object, &s0);
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let s = Shape::Sphere(Sphere::new(
            Default::default(),
            Transform::translation(5.0, 0.0, 0.0),
        ));

        let g = Group::new([s], Transform::scaling(2.0, 2.0, 2.0).unwrap());

        let r = Ray {
            origin: Point::new(10.0, 0.0, -10.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = g.intersect(&r);

        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn a_group_applies_its_transformation_to_its_children() {
        let sphere_transform = Transform::translation(2.0, 2.0, 2.0);
        let subgroup_transform = Transform::rotation_y(std::f64::consts::FRAC_PI_3);
        let group_transform = Transform::scaling(4.0, 4.0, 4.0).unwrap();

        let s = Shape::Sphere(Sphere::new(Default::default(), sphere_transform));

        let g1 = Group::new([s], subgroup_transform);

        let g0 = Group::new([Shape::Group(g1)], group_transform);

        let s = get_subgroup_child(&g0);

        assert_eq!(
            s.as_ref().transform,
            group_transform * subgroup_transform * sphere_transform
        );
    }

    #[test]
    fn a_group_has_a_bounding_box_that_contains_its_children() {
        let s = Shape::Sphere(Sphere::new(
            Default::default(),
            Transform::translation(2.0, 5.0, -3.0) * Transform::scaling(2.0, 2.0, 2.0).unwrap(),
        ));

        let c = Shape::Cylinder(Cylinder::new(
            Default::default(),
            Transform::translation(-4.0, -1.0, 4.0) * Transform::scaling(0.5, 1.0, 0.5).unwrap(),
            -2.0,
            2.0,
            false,
        ));

        let g = Group::new([s, c], Default::default());

        let bbox = g.props.local_bounds;

        assert_eq!(bbox.min, Point::new(-4.5, -3.0, -5.0));
        assert_eq!(bbox.max, Point::new(4.0, 7.0, 4.5));
    }

    #[test]
    fn a_ray_doesnt_test_a_groups_children_if_its_bounding_box_is_missed() {
        let g = Group::new([Shape::Sphere(Default::default())], Default::default());

        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        assert!(g.intersect(&r).is_empty());
    }

    #[test]
    fn a_ray_test_a_groups_children_if_it_hits_its_bounding_box() {
        let g = Group::new([Shape::Sphere(Default::default())], Default::default());

        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        assert!(!g.intersect(&r).is_empty());
    }

    #[test]
    fn partitioning_a_groups_children() {
        let s0 = Shape::Sphere(Sphere::new(
            Default::default(),
            Transform::translation(-2.0, 0.0, 0.0),
        ));

        let s1 = Shape::Sphere(Sphere::new(
            Default::default(),
            Transform::translation(2.0, 0.0, 0.0),
        ));

        let s2 = Shape::Sphere(Default::default());

        let mut g = Group::new([s0, s1, s2], Default::default());

        let s0 = g.children[0].clone();
        let s1 = g.children[1].clone();
        let s2 = g.children[2].clone();

        let (left, right) = g.partition_children();

        assert_eq!(g, Group::new([s2], Default::default()));
        assert_eq!(left, vec![s0]);
        assert_eq!(right, vec![s1]);
    }

    #[test]
    fn subdividing_a_group_partitions_its_children() {
        let s0 = Shape::Sphere(Sphere::new(
            Default::default(),
            Transform::translation(-2.0, -2.0, 0.0),
        ));

        let s1 = Shape::Sphere(Sphere::new(
            Default::default(),
            Transform::translation(-2.0, 2.0, 0.0),
        ));

        let s2 = Shape::Sphere(Sphere::new(
            Default::default(),
            Transform::scaling(4.0, 4.0, 4.0).unwrap(),
        ));

        let mut g = Group::new([s0, s1, s2], Default::default());

        let s0 = g.children[0].clone();
        let s1 = g.children[1].clone();
        let s2 = g.children[2].clone();

        g.divide(1);

        let subgroup = match &g.children[1] {
            Shape::Group(subgroup) => subgroup,
            _ => unimplemented!(),
        };

        assert_eq!(g.children[0], s2);
        assert_eq!(subgroup.children.len(), 2);
        assert_eq!(
            subgroup.children[0],
            Shape::Group(Group::new([s0], Default::default()))
        );
        assert_eq!(
            subgroup.children[1],
            Shape::Group(Group::new([s1], Default::default()))
        );
    }

    #[test]
    fn subdividing_a_group_with_too_few_children() {
        let s0 = Shape::Sphere(Sphere::new(
            Default::default(),
            Transform::translation(-2.0, 0.0, 0.0),
        ));

        let s1 = Shape::Sphere(Sphere::new(
            Default::default(),
            Transform::translation(2.0, 1.0, 0.0),
        ));

        let s2 = Shape::Sphere(Sphere::new(
            Default::default(),
            Transform::translation(2.0, -1.0, 0.0),
        ));

        let subgroup = Shape::Group(Group::new(
            [s0.clone(), s1.clone(), s2.clone()],
            Default::default(),
        ));

        let s3 = Shape::Sphere(Default::default());

        let mut g = Group::new([subgroup.clone(), s3.clone()], Default::default());

        g.divide(3);

        let subgroup = match &g.children[0] {
            Shape::Group(subgroup) => subgroup,
            _ => unimplemented!(),
        };

        assert_eq!(g.children[1], s3);
        assert_eq!(subgroup.children.len(), 2);
        assert_eq!(
            subgroup.children[0],
            Shape::Group(Group::new([s0], Default::default()))
        );
        assert_eq!(
            subgroup.children[1],
            Shape::Group(Group::new([s2, s1], Default::default()))
        );
    }
}
