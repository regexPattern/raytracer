use crate::{intersection::Intersection, ray::Ray, transform::Transform};

use super::{bounding_box::BoundingBox, object::ObjectCache, Shape};

/// Cluster of multiple shapes.
///
/// # Examples
///
/// A group must be built from a [GroupBuilder].
///
/// Building a group and adding children to it.
///
/// ```
/// use raytracer::{
///     shape::{Group, GroupBuilder, Shape},
///     transform::Transform,
/// };
///
/// // A group can be created with children inside.
/// let mut group = Group::from(GroupBuilder {
///     children: [Shape::Sphere(Default::default())],
///     transform: Transform::scaling(1.0, 2.0, 3.0).unwrap(),
/// });
///
/// // You can also add individual childs afterwards.
/// group.push(Shape::Cube(Default::default()));
///
/// // Or you can add multiple children at once.
/// group.extend([
///     Shape::Plane(Default::default()),
///     Shape::Sphere(Default::default()),
/// ]);
/// ```
///
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Group {
    pub(crate) children: Vec<Shape>,
    pub(crate) object_cache: ObjectCache,
}

/// Builder for a group.
#[derive(Debug)]
pub struct GroupBuilder<T: IntoIterator<Item = Shape>> {
    /// Initial children of the group.
    pub children: T,

    /// Transformation of the group. This transforms all it's children alongside it.
    pub transform: Transform,
}

impl<T> From<GroupBuilder<T>> for Group
where
    T: IntoIterator<Item = Shape>,
{
    fn from(builder: GroupBuilder<T>) -> Self {
        let mut group = Self {
            children: vec![],
            object_cache: ObjectCache {
                transform: builder.transform,
                transform_inverse: builder.transform.inverse(),
                ..Default::default()
            },
        };

        group.extend(builder.children);
        group
    }
}

impl Group {
    /// Add a child to the group.
    pub fn push(&mut self, mut child: Shape) {
        Self::apply_transform_to_child(&mut child, self.object_cache.transform);
        self.object_cache
            .bounding_box
            .merge(child.as_ref().parent_space_bounding_box);

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
        child.as_mut().parent_space_bounding_box =
            child.as_ref().bounding_box.transform(new_transform);
    }

    /// Add multiple children at once.
    pub fn extend<T>(&mut self, children: T)
    where
        T: IntoIterator<Item = Shape>,
    {
        for child in children {
            self.push(child);
        }
    }

    pub(crate) fn local_intersect(&self, ray: &Ray) -> Vec<Intersection<'_>> {
        if !self.bounding_box().intersect(ray) {
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

    /// Divide the group into multiple subgroups.
    ///
    /// This can significantly improve the performance of scenes with a large number of objects,
    /// for example, 3D models, by using [bounding volumes
    /// hierarchy](https://en.wikipedia.org/wiki/Bounding_volume_hierarchy) optimization, which
    /// prevents unnecesarry collition checks for each casted ray.
    ///
    /// Try with different threshold values to see what division level works best for your group.
    ///
    /// # Arguments
    ///
    /// * `threshold` - The maximum number of children that a subgroup will have after dividing
    /// their parent group.
    ///
    /// # Examples
    ///
    /// Dividing a group of `3000` spheres into `10` groups of `300` spheres. This allows each
    /// group of `300` spheres to be checked for collition individually instead of always testing
    /// all `3000` spheres. If the ray doesn't intersect a group iteself, it doesn't have to also
    /// check for all of that group's children, thus, optimizing rendering times.
    ///
    /// ```
    /// use raytracer::{
    ///     model::{Model, OBJModelBuilder},
    ///     shape::{Group, GroupBuilder, Shape, ShapeBuilder, Sphere},
    ///     transform::Transform,
    /// };
    ///
    /// let mut group = Group::from(GroupBuilder {
    ///     children: [],
    ///     transform: Default::default(),
    /// });
    ///
    /// // Create a discrete row of 3000 spheres.
    /// for i in 0..3000 {
    ///     let move_sphere = Transform::translation(f64::from(i) * 3.0, 0.0, 0.0);
    ///     group.push(Shape::Sphere(Sphere::from(ShapeBuilder {
    ///         transform: move_sphere,
    ///         ..Default::default()
    ///     })));
    /// }
    ///
    /// // Now every time a ray is casted, the spheres will be checked in groups of 300 instead of
    /// // always checking all the spheres.
    /// group.divide(300);
    /// ```
    ///
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
        let (left_box, right_box) = self.bounding_box().split();

        let mut left_children = Vec::with_capacity(self.children.len());
        let mut right_children = Vec::with_capacity(self.children.len());

        let mut i = 0;
        while i < self.children.len() {
            let child = &mut self.children[i];
            let child_bounding_box = child.as_ref().parent_space_bounding_box;

            if left_box.contains(&child_bounding_box) {
                child.as_mut().transform =
                    self.object_cache.transform_inverse * child.as_ref().transform;
                left_children.push(self.children.swap_remove(i));
            } else if right_box.contains(&child_bounding_box) {
                child.as_mut().transform =
                    self.object_cache.transform_inverse * child.as_ref().transform;
                right_children.push(self.children.swap_remove(i));
            } else {
                i += 1;
            }
        }

        let mut adjusted_bounding_box = BoundingBox::default();
        for child in &self.children {
            let child_bounding_box = child.as_ref().parent_space_bounding_box;
            adjusted_bounding_box.merge(child_bounding_box);
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

    fn bounding_box(&self) -> BoundingBox {
        let mut bounding_box = BoundingBox::default();

        for child in &self.children {
            let child_bounding_box = child.as_ref().parent_space_bounding_box;
            bounding_box.merge(child_bounding_box);
        }

        bounding_box
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        shape::{
            cylinder::{Cylinder, CylinderBuilder},
            sphere::Sphere,
            ShapeBuilder,
        },
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
        let child1 = Shape::Sphere(Sphere::from(ShapeBuilder {
            transform: Transform::translation(0.0, 0.0, -3.0),
            ..Default::default()
        }));
        let child2 = Shape::Sphere(Sphere::from(ShapeBuilder {
            transform: Transform::translation(5.0, 0.0, 0.0),
            ..Default::default()
        }));

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
        let child = Shape::Sphere(Sphere::from(ShapeBuilder {
            transform: Transform::translation(5.0, 0.0, 0.0),
            ..Default::default()
        }));

        let group = Group::from(GroupBuilder {
            children: [child],
            transform: Transform::scaling(2.0, 2.0, 2.0).unwrap(),
        });

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
        let s0 = Shape::Sphere(Sphere::from(ShapeBuilder {
            transform: Transform::translation(2.0, 5.0, -3.0)
                * Transform::scaling(2.0, 2.0, 2.0).unwrap(),
            ..Default::default()
        }));

        let s1 = Shape::Cylinder(Cylinder::from(CylinderBuilder {
            transform: Transform::translation(-4.0, -1.0, 4.0)
                * Transform::scaling(0.5, 1.0, 0.5).unwrap(),
            min: -2.0,
            max: 2.0,
            ..Default::default()
        }));

        let group = Group::from(GroupBuilder {
            children: [s0, s1],
            transform: Transform::scaling(2.0, 2.0, 2.0).unwrap(),
        });

        let bounding_box = group.bounding_box();

        assert_eq!(bounding_box.min, Point::new(-9.0, -6.0, -10.0));
        assert_eq!(bounding_box.max, Point::new(8.0, 14.0, 9.0));
    }

    #[test]
    fn partitioning_a_groups_children() {
        let s0 = Shape::Sphere(Sphere::from(ShapeBuilder {
            transform: Transform::translation(-2.0, 0.0, 0.0),
            ..Default::default()
        }));
        let s1 = Shape::Sphere(Sphere::from(ShapeBuilder {
            transform: Transform::translation(2.0, 0.0, 0.0),
            ..Default::default()
        }));
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
        let s0 = Shape::Sphere(Sphere::from(ShapeBuilder {
            transform: Transform::translation(-2.0, 0.0, 0.0),
            ..Default::default()
        }));
        let s1 = Shape::Sphere(Sphere::from(ShapeBuilder {
            transform: Transform::translation(2.0, 0.0, 0.0),
            ..Default::default()
        }));
        let s2 = Shape::Sphere(Sphere::from(ShapeBuilder {
            transform: Transform::scaling(4.0, 4.0, 4.0).unwrap(),
            ..Default::default()
        }));

        let mut group = Group::from(GroupBuilder {
            children: [&s0, &s1, &s2].into_iter().cloned(),
            transform: Default::default(),
        });

        group.divide(1);

        assert_eq!(group.children[0], s2);

        let left_subgroup = match &group.children[1] {
            Shape::Group(subgroup) => subgroup,
            _ => panic!(),
        };

        let right_subgroup = match &group.children[2] {
            Shape::Group(subgroup) => subgroup,
            _ => panic!(),
        };

        assert_eq!(left_subgroup.children, vec![s0]);
        assert_eq!(right_subgroup.children, vec![s1]);
    }
}
