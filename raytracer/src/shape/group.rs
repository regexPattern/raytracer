use crate::{intersection::Intersection, ray::Ray, transform::Transform};

use super::{BoundingBox, Shape};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Group {
    pub children: Vec<Shape>,
    pub(crate) transform: Transform,
}

impl Group {
    pub fn new<T>(children: T, transform: Transform) -> Self
    where
        T: Into<Vec<Shape>>,
    {
        let mut children = children.into();

        children
            .iter_mut()
            .for_each(|child| Self::apply_transform_to_child(transform, child));

        Self {
            children,
            transform,
        }
    }

    fn apply_transform_to_child(transform: Transform, child: &mut Shape) {
        match child {
            Shape::Group(group) => group
                .children
                .iter_mut()
                .for_each(|subchild| Self::apply_transform_to_child(transform, subchild)),
            _ => child.set_transform(transform * child.get_transform()),
        }
    }

    pub(crate) fn update_transform(&mut self, transform: Transform) {
        self.transform = transform;
        self.children
            .iter_mut()
            .for_each(|child| Self::apply_transform_to_child(self.transform, child));
    }

    pub fn add_child(&mut self, mut child: Shape) {
        Self::apply_transform_to_child(self.transform, &mut child);
        self.children.push(child);
    }

    pub fn add_children<T>(&mut self, children: T)
    where
        T: IntoIterator<Item = Shape>,
    {
        for child in children {
            self.add_child(child);
        }
    }

    pub fn add_subgroup<T>(&mut self, subgroup: T)
    where
        T: Into<Vec<Shape>>,
    {
        let subgroup = Self::new(subgroup, Default::default());
        self.add_child(Shape::Group(subgroup))
    }

    pub(crate) fn intersect(&self, ray: &Ray) -> Vec<Intersection<'_>> {
        if !self.bounding_box().intersect(&ray) {
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

    pub fn bounding_box(&self) -> BoundingBox {
        let mut bbox = BoundingBox::default();

        for child in &self.children {
            let child_bbox = child.get_bounding_box();
            bbox.merge(child_bbox);
        }

        bbox
    }

    pub fn partition_children(&mut self) -> (Vec<Shape>, Vec<Shape>) {
        let (left_bbox, right_bbox) = self.bounding_box().split();

        let mut left_children = vec![];
        let mut right_children = vec![];

        let mut i = 0;
        while i < self.children.len() {
            let child_bbox = self.children[i].get_bounding_box();

            if left_bbox.contains_box(&child_bbox) {
                left_children.push(self.children.swap_remove(i));
            } else if right_bbox.contains_box(&child_bbox) {
                right_children.push(self.children.swap_remove(i));
            } else {
                i += 1;
            }
        }

        (left_children, right_children)
    }

    pub fn divide(&mut self, threshold: usize) {
        if threshold <= self.children.len() {
            let (left, right) = self.partition_children();

            if !left.is_empty() {
                self.add_subgroup(left);
            }

            if !right.is_empty() {
                self.add_subgroup(right);
            }
        }

        for child in &mut self.children {
            child.divide(threshold)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        shape::{BaseShape, Cylinder},
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

        let s1 = Shape::Sphere(BaseShape {
            transform: Transform::translation(0.0, 0.0, -3.0),
            ..Default::default()
        });

        let s2 = Shape::Sphere(BaseShape {
            transform: Transform::translation(5.0, 0.0, 0.0),
            ..Default::default()
        });

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
        let s = Shape::Sphere(BaseShape {
            transform: Transform::translation(5.0, 0.0, 0.0),
            ..Default::default()
        });

        let g = Group::new([s], Transform::try_scaling(2.0, 2.0, 2.0).unwrap());

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
        let group_transform = Transform::try_scaling(4.0, 4.0, 4.0).unwrap();

        let s = Shape::Sphere(BaseShape {
            transform: sphere_transform,
            ..Default::default()
        });

        let g2 = Group::new([s], subgroup_transform);

        let g1 = Group::new([Shape::Group(g2)], group_transform);

        let s = get_subgroup_child(&g1);

        assert_eq!(
            s.get_transform(),
            group_transform * subgroup_transform * sphere_transform
        );
    }

    #[test]
    fn a_group_has_a_bounding_box_that_contains_its_children() {
        let s = Shape::Sphere(BaseShape {
            transform: Transform::translation(2.0, 5.0, -3.0)
                * Transform::try_scaling(2.0, 2.0, 2.0).unwrap(),
            ..Default::default()
        });

        let c = Shape::Cylinder(Cylinder {
            base_shape: BaseShape {
                transform: Transform::translation(-4.0, -1.0, 4.0)
                    * Transform::try_scaling(0.5, 1.0, 0.5).unwrap(),
                ..Default::default()
            },
            min: -2.0,
            max: 2.0,
            ..Default::default()
        });

        let g = Group::new([s, c], Default::default());

        let bbox = g.bounding_box();

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
        let s0 = Shape::Sphere(BaseShape {
            transform: Transform::translation(-2.0, 0.0, 0.0),
            ..Default::default()
        });

        let s1 = Shape::Sphere(BaseShape {
            transform: Transform::translation(2.0, 0.0, 0.0),
            ..Default::default()
        });

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
    fn creating_a_subgroup_from_a_list_of_children() {
        let s0 = Shape::Sphere(Default::default());
        let s1 = Shape::Cube(Default::default());

        let mut g = Group::default();

        g.add_subgroup([s0.clone(), s1.clone()]);

        assert_eq!(g.children.len(), 1);
        assert_eq!(
            g.children[0],
            Shape::Group(Group::new([s0, s1], Default::default()))
        );
    }

    #[test]
    fn subdividing_a_group_partitions_its_children() {
        let s0 = Shape::Sphere(BaseShape {
            transform: Transform::translation(-2.0, -2.0, 0.0),
            ..Default::default()
        });

        let s1 = Shape::Sphere(BaseShape {
            transform: Transform::translation(-2.0, 2.0, 0.0),
            ..Default::default()
        });

        let s2 = Shape::Sphere(BaseShape {
            transform: Transform::try_scaling(4.0, 4.0, 4.0).unwrap(),
            ..Default::default()
        });

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
        let s0 = Shape::Sphere(BaseShape {
            transform: Transform::translation(-2.0, 0.0, 0.0),
            ..Default::default()
        });

        let s1 = Shape::Sphere(BaseShape {
            transform: Transform::translation(2.0, 1.0, 0.0),
            ..Default::default()
        });

        let s2 = Shape::Sphere(BaseShape {
            transform: Transform::translation(2.0, -1.0, 0.0),
            ..Default::default()
        });

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
