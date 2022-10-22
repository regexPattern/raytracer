use crate::float;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::tuple::{Point, Vector};

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub object: Shape,
    pub t: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct MetaData {
    pub i: Intersection,
    pub eyev: Vector,
    pub inside: bool,
    pub normalv: Vector,
    pub point: Point,
    pub over_point: Point,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        float::approx(self.t, other.t) && self.object == other.object
    }
}

impl Intersection {
    pub fn comps(self, ray: Ray) -> MetaData {
        let Self { t, object } = self;
        let point = ray.position(t);
        let eyev = -ray.direction;
        let normalv = object.normal_at(point);
        let inside = normalv.dot(eyev) < 0.0;

        let normalv = if inside { -1.0 } else { 1.0 } * normalv;

        let over_point = point + normalv * crate::float::EPSILON;

        MetaData {
            i: self,
            eyev,
            inside,
            normalv,
            point,
            over_point,
        }
    }

    pub fn hit(mut xs: Vec<Self>) -> Option<Self> {
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs.into_iter().find(|i| i.t.is_sign_positive())
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;
    use crate::matrix::Matrix;
    use crate::shape::ShapeProps;

    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let shape = Shape::Sphere(ShapeProps::default());

        let i = Intersection {
            t: 3.5,
            object: shape,
        };

        assert_approx!(i.t, 3.5);
        assert_eq!(i.object, shape);
    }

    #[test]
    fn aggregating_intersections() {
        let shape = Shape::Sphere(ShapeProps::default());

        let i1 = Intersection {
            t: 1.0,
            object: shape,
        };
        let i2 = Intersection {
            t: 2.0,
            object: shape,
        };

        let xs = vec![i1, i2];

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0].t, 1.0);
        assert_approx!(xs[1].t, 2.0);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let shape = Shape::Sphere(ShapeProps::default());

        let i1 = Intersection {
            t: 1.0,
            object: shape,
        };
        let i2 = Intersection {
            t: 2.0,
            object: shape,
        };

        let xs = vec![i2, i1];

        let i = Intersection::hit(xs);

        assert_eq!(i, Some(i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let shape = Shape::Sphere(ShapeProps::default());

        let i1 = Intersection {
            t: -1.0,
            object: shape,
        };
        let i2 = Intersection {
            t: 1.0,
            object: shape,
        };

        let xs = vec![i2, i1];

        let i = Intersection::hit(xs);

        assert_eq!(i, Some(i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let shape = Shape::Sphere(ShapeProps::default());

        let i1 = Intersection {
            t: -2.0,
            object: shape,
        };
        let i2 = Intersection {
            t: -1.0,
            object: shape,
        };

        let xs = vec![i2, i1];

        let i = Intersection::hit(xs);

        assert_eq!(i, None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let shape = Shape::Sphere(ShapeProps::default());

        let i1 = Intersection {
            t: 5.0,
            object: shape,
        };
        let i2 = Intersection {
            t: 7.0,
            object: shape,
        };
        let i3 = Intersection {
            t: -3.0,
            object: shape,
        };
        let i4 = Intersection {
            t: 2.0,
            object: shape,
        };

        let xs = vec![i1, i2, i3, i4];

        let i = Intersection::hit(xs);

        assert_eq!(i, Some(i4));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let shape = Shape::Sphere(ShapeProps::default());

        let i = Intersection {
            t: 4.0,
            object: shape,
        };

        let comps = i.comps(ray);

        assert_eq!(comps.i.t, i.t);
        assert_eq!(comps.i.object, i.object);
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let shape = Shape::Sphere(ShapeProps::default());

        let i = Intersection {
            t: 4.0,
            object: shape,
        };

        let comps = i.comps(ray);

        assert!(!comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let shape = Shape::Sphere(ShapeProps::default());

        let i = Intersection {
            t: 1.0,
            object: shape,
        };

        let comps = i.comps(ray);

        assert!(comps.inside);
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let shape = Shape::Sphere(ShapeProps {
            transform: Matrix::translation(0.0, 0.0, 1.0),
            ..Default::default()
        });

        let i = Intersection {
            t: 5.0,
            object: shape,
        };

        let comps = i.comps(ray);

        assert!(comps.over_point.0.z < -crate::float::EPSILON / 2.0);
        assert!(comps.point.0.z > -crate::float::EPSILON);
    }
}
