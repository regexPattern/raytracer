use crate::{intersection::Intersection, ray::Ray, transform::Transform};

use super::Shape;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Group {
    pub(crate) children: Vec<Shape>,
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

    pub fn extend<T>(&mut self, children: T)
    where
        T: IntoIterator<Item = Shape>
    {
        for child in children {
            self.add_child(child);
        }
    }

    pub(crate) fn intersect(&self, ray: &Ray) -> Vec<Intersection<'_>> {
        let mut intersections: Vec<_> = self
            .children
            .iter()
            .flat_map(|child| child.intersect(ray))
            .collect();

        Intersection::sort(&mut intersections);
        intersections
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        shape::BaseShape,
        tuple::{Point, Vector},
    };

    use super::*;

    fn get_subgroup_child(super_group: &Group) -> &Shape {
        match &super_group.children[0] {
            Shape::Group(sub_group) => &sub_group.children[0],
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
}
