use crate::ray::Ray;
use crate::shape::{Intersectable, Shape};
use crate::tuple::{Point, Vector};
use crate::utils;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub object: Shape,
}

#[derive(Copy, Clone, Debug)]
pub struct ComputedIntersection {
    pub intersection: Intersection,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub over_point: Point,
    inside: bool,
}

impl Intersection {
    pub fn new(t: f64, object: Shape) -> Self {
        Self { t, object }
    }

    pub fn hit(mut xs: Vec<Self>) -> Option<Self> {
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs.into_iter().find(|i| i.t.is_sign_positive())
    }

    pub fn prepare_computations(self, ray: Ray) -> ComputedIntersection {
        let point = ray.position(self.t);
        let eyev = -ray.direction;
        let mut normalv = self.object.normal_at(point);
        let inside = normalv.dot(eyev) < 0.0;

        if inside {
            normalv = -normalv;
        }

        let over_point = point + normalv * utils::EPSILON;

        ComputedIntersection {
            intersection: self,
            point,
            eyev,
            normalv,
            inside,
            over_point,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::shape::Sphere;
    use crate::transformation;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        // TODO: Ver si puedo implementar `Default` para una enum.
        let s = Shape::Sphere(Sphere::default());
        let i = Intersection::new(3.5, s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Shape::Sphere(Sphere::default());
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);

        let xs = vec![i1, i2];

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Shape::Sphere(Sphere::default());

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Shape::Sphere(Sphere::default());
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = vec![i2, i1];

        let i = Intersection::hit(xs);

        assert_eq!(i, Some(i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Shape::Sphere(Sphere::default());
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s);
        let xs = vec![i2, i1];

        let i = Intersection::hit(xs);

        assert_eq!(i, Some(i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Shape::Sphere(Sphere::default());
        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s);
        let xs = vec![i2, i1];

        let i = Intersection::hit(xs);

        assert_eq!(i, None)
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Shape::Sphere(Sphere::default());
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
        let xs = vec![i1, i2, i3, i4];

        let i = Intersection::hit(xs);

        assert_eq!(i, Some(i4));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Shape::Sphere(Sphere::default());
        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computations(r);

        assert_eq!(comps.intersection.object, i.object);
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Shape::Sphere(Sphere::default());
        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computations(r);

        assert!(!comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Shape::Sphere(Sphere::default());
        let i = Intersection::new(1.0, shape);

        let comps = i.prepare_computations(r);

        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Shape::Sphere(Sphere {
            transform: transformation::translation(0.0, 0.0, 1.0),
            ..Sphere::default()
        });
        let i = Intersection::new(5.0, shape);

        let comps = i.prepare_computations(r);

        assert!(comps.over_point.0.z < -utils::EPSILON / 2.0);
        assert!(comps.point.0.z > comps.over_point.0.z);
    }
}
