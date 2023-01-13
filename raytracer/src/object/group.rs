use std::cmp::Ordering;

use crate::{float, intersections::Intersection, ray::Ray, transform::Transform};

use super::Object;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Group {
    pub children: Vec<Object>,
    pub transform: Transform,
}

impl Group {
    pub fn add_child(&mut self, mut child: Object) {
        if let Object::Group(group) = &mut child {
            group
                .children
                .iter_mut()
                .for_each(|child| *child.transform_mut() = self.transform * child.transform());
        } else {
            *child.transform_mut() = self.transform * child.transform();
        }

        self.children.push(child);
    }

    pub(crate) fn local_intersect(&self, ray: &Ray) -> Vec<Intersection<'_>> {
        let mut intersections: Vec<_> = self
            .children
            .iter()
            .flat_map(|child| child.intersect(ray))
            .collect();

        // TODO: Fix this in the Intersections refactor. (unify this).
        intersections.sort_unstable_by(|i1, i2| {
            if float::approx(i1.t, i2.t) {
                Ordering::Equal
            } else if i1.t < i2.t {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        intersections
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        object::{sphere::Sphere, Object},
        transform::Transform,
        tuple::{Point, Vector},
    };

    use super::*;

    fn test_object() -> Object {
        Object::Sphere(Default::default())
    }

    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let g = Group::default();

        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = g.local_intersect(&r);

        assert!(xs.is_empty());
    }

    #[test]
    fn intersecting_a_ray_with_a_non_empty_group() {
        let s1 = test_object();

        let s2 = Object::Sphere(Sphere {
            transform: Transform::translation(0.0, 0.0, -3.0),
            ..Default::default()
        });

        let s3 = Object::Sphere(Sphere {
            transform: Transform::translation(5.0, 0.0, 0.0),
            ..Default::default()
        });

        let g = Group {
            children: vec![s1.clone(), s2.clone(), s3.clone()],
            transform: Default::default(),
        };

        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = g.local_intersect(&r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object, &s2);
        assert_eq!(xs[1].object, &s2);
        assert_eq!(xs[2].object, &s1);
        assert_eq!(xs[3].object, &s1);
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let s = Object::Sphere(Sphere {
            transform: Transform::translation(5.0, 0.0, 0.0),
            ..Default::default()
        });

        let g = Object::Group(Group {
            children: vec![s],
            transform: Transform::try_scaling(2.0, 2.0, 2.0).unwrap(),
        });

        let r = Ray {
            origin: Point::new(10.0, 0.0, -10.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = g.intersect(&r);

        assert_eq!(xs.len(), 2);
    }
}
