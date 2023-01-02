use std::cmp::Ordering;

use crate::{
    ray::Ray,
    sphere::Sphere,
    tuple::{Point, Vector},
    utils,
};

#[derive(Clone, Debug)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

#[derive(Debug)]
pub(crate) struct Computation<'a, 'b> {
    pub intersection: &'a Intersection<'b>,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Self) -> bool {
        utils::approx(self.t, other.t) && self.object == other.object
    }
}

impl Intersection<'_> {
    pub(crate) fn sort(xs: &mut [Self]) {
        xs.sort_unstable_by(|i1, i2| {
            if utils::approx(i1.t, i2.t) {
                Ordering::Equal
            } else if i1.t < i2.t {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
    }

    pub fn hit(mut xs: Vec<Self>) -> Option<Self> {
        Self::sort(&mut xs);
        xs.into_iter().find(|i| i.t > 0.0)
    }

    pub(crate) fn prepare_computations(&self, ray: &Ray) -> Computation<'_, '_> {
        let intersection = self;
        let point = ray.position(self.t);
        let eyev = -ray.direction;

        let normalv = self.object.normal_at(point);
        let inside = normalv.dot(eyev) < 0.0;

        let normalv = if inside { -normalv } else { normalv };

        Computation {
            intersection,
            point,
            eyev,
            normalv,
            inside,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_approx,
        ray::Ray,
        tuple::{Point, Vector},
    };

    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::default();

        let i = Intersection { t: 3.5, object: &s };

        assert_approx!(i.t, 3.5);
        assert_eq!(i.object, &s);
    }

    #[test]
    fn aggregating_intesrections() {
        let s = Sphere::default();

        let i1 = Intersection { t: 1.0, object: &s };
        let i2 = Intersection { t: 2.0, object: &s };

        let xs = vec![&i1, &i2];

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0].t, 1.0);
        assert_approx!(xs[1].t, 2.0);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::default();

        let i1 = Intersection { t: 1.0, object: &s };
        let i2 = Intersection { t: 2.0, object: &s };

        let xs = vec![i1.clone(), i2];

        let i = Intersection::hit(xs);

        assert_eq!(i, Some(i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default();

        let i1 = Intersection {
            t: -1.0,
            object: &s,
        };
        let i2 = Intersection { t: 1.0, object: &s };

        let xs = vec![i1, i2.clone()];

        let i = Intersection::hit(xs);

        assert_eq!(i, Some(i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::default();

        let i1 = Intersection {
            t: -2.0,
            object: &s,
        };
        let i2 = Intersection {
            t: -1.0,
            object: &s,
        };

        let xs = vec![i1, i2];

        let i = Intersection::hit(xs);

        assert_eq!(i, None);
    }

    #[test]
    fn sorting_a_vector_of_intersections() {
        let s = Sphere::default();

        let i1 = Intersection { t: 5.0, object: &s };
        let i2 = Intersection { t: 7.0, object: &s };
        let i3 = Intersection {
            t: -3.0,
            object: &s,
        };
        let i4 = Intersection { t: 2.0, object: &s };

        let mut xs = vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()];

        Intersection::sort(&mut xs);

        assert_eq!(xs[0], i3);
        assert_eq!(xs[1], i4);
        assert_eq!(xs[2], i1);
        assert_eq!(xs[3], i2);
    }

    #[test]
    fn the_hit_is_always_the_lowest_non_negative_intersection() {
        let s = Sphere::default();

        let i1 = Intersection { t: 5.0, object: &s };
        let i2 = Intersection { t: 7.0, object: &s };
        let i3 = Intersection {
            t: -3.0,
            object: &s,
        };
        let i4 = Intersection { t: 2.0, object: &s };

        let xs = vec![i1, i2, i3, i4.clone()];

        let i = Intersection::hit(xs);

        assert_eq!(i, Some(i4));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Sphere::default();

        let i = Intersection { t: 4.0, object: &s };

        let comps = i.prepare_computations(&r);

        assert_approx!(comps.intersection.t, 4.0);
        assert_eq!(comps.intersection.object, &s);
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Sphere::default();

        let i = Intersection { t: 4.0, object: &s };

        let comps = i.prepare_computations(&r);

        assert!(!comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Sphere::default();

        let i = Intersection { t: 1.0, object: &s };

        let comps = i.prepare_computations(&r);

        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }
}
