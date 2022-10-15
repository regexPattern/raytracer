use crate::float;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::{Point, Vector};

#[derive(Clone, Debug)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

pub struct PreparedComputation<'a> {
    pub t: f64,
    pub object: &'a Sphere,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Self) -> bool {
        float::approx(self.t, other.t) && self.object == other.object
    }
}

impl Intersection<'_> {
    pub fn hit(mut xs: Vec<Intersection<'_>>) -> Option<Intersection> {
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs.into_iter().find(|i| i.t.is_sign_positive())
    }
}

impl<'a> PreparedComputation<'a> {
    pub fn new(i: &'a Intersection, ray: &Ray) -> Self {
        let Intersection { t, object } = *i;
        let point = ray.position(t);
        let eyev = -ray.direction;
        let normalv = object.normal_at(point);
        let inside = normalv.dot(eyev) < 0.0;

        let normalv = if inside { -1.0 } else { 1.0 } * normalv;

        Self {
            t,
            object,
            point,
            eyev,
            normalv,
            inside,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;

    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::default();

        let i = Intersection { t: 3.5, object: &s };

        assert_approx!(i.t, 3.5);
        assert_eq!(i.object, &s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::default();
        let i1 = Intersection { t: 1.0, object: &s };
        let i2 = Intersection { t: 2.0, object: &s };

        let xs = vec![i1, i2];

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0].t, 1.0);
        assert_approx!(xs[1].t, 2.0);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::default();
        let i1 = Intersection { t: 1.0, object: &s };
        let i2 = Intersection { t: 2.0, object: &s };
        // TODO: Maybe I should make a vector of references instead????
        // It's probably more convenient.
        let xs = vec![i2, i1.clone()];

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
        let xs = vec![i2.clone(), i1];

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
        let xs = vec![i2, i1];

        let i = Intersection::hit(xs);

        assert_eq!(i, None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
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

        let shape = Sphere::default();
        let i = Intersection {
            t: 4.0,
            object: &shape,
        };

        let comps = PreparedComputation::new(&i, &r);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
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

        let shape = Sphere::default();
        let i = Intersection {
            t: 4.0,
            object: &shape,
        };

        let comps = PreparedComputation::new(&i, &r);

        assert!(!comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let shape = Sphere::default();
        let i = Intersection {
            t: 1.0,
            object: &shape,
        };

        let comps = PreparedComputation::new(&i, &r);

        assert!(comps.inside);
    }
}
