use crate::{
    float,
    ray::Ray,
    shape::Shapes,
    tuple::{Point, Vector},
};

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub object: Shapes,
    pub t: f64,
}

#[derive(Debug)]
pub struct Computation {
    pub i: Intersection,
    pub eyev: Vector,
    pub inside: bool,
    pub normalv: Vector,
    pub over_point: Point,
    pub point: Point,
    pub reflectv: Vector,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        float::approx(self.t, other.t) && self.object == other.object
    }
}

impl Intersection {
    pub fn comps(self, ray: &Ray) -> Computation {
        let Self { t, object } = self;
        let point = ray.position(t);
        let eyev = -ray.direction;
        let normalv = object.normal_at(point);
        let inside = normalv.dot(eyev) < 0.0;

        let normalv = if inside { -1.0 } else { 1.0 } * normalv;

        let over_point = point + normalv * crate::float::EPSILON;
        let reflectv = ray.direction.reflect(normalv);

        Computation {
            i: self,
            eyev,
            inside,
            normalv,
            over_point,
            point,
            reflectv,
        }
    }

    pub fn hit(mut xs: Vec<Self>) -> Option<Self> {
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        xs.into_iter().find(|i| i.t.is_sign_positive())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_approx,
        matrix::Matrix,
        shape::{Figure, Plane, Sphere},
    };

    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let shape = Shapes::Sphere(Sphere::default());

        let i = Intersection {
            object: shape,
            t: 3.5,
        };

        assert_eq!(i.object, shape);
        assert_approx!(i.t, 3.5);
    }

    #[test]
    fn aggregating_intersections() {
        let shape = Shapes::Sphere(Sphere::default());

        let i1 = Intersection {
            object: shape,
            t: 1.0,
        };

        let i2 = Intersection {
            object: shape,
            t: 2.0,
        };

        let xs = vec![i1, i2];

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0].t, 1.0);
        assert_approx!(xs[1].t, 2.0);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let shape = Shapes::Sphere(Sphere::default());

        let i1 = Intersection {
            object: shape,
            t: 1.0,
        };

        let i2 = Intersection {
            object: shape,
            t: 2.0,
        };

        let xs = vec![i2, i1];

        let i = Intersection::hit(xs);

        assert_eq!(i, Some(i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let shape = Shapes::Sphere(Sphere::default());

        let i1 = Intersection {
            object: shape,
            t: -1.0,
        };

        let i2 = Intersection {
            object: shape,
            t: 1.0,
        };

        let xs = vec![i2, i1];

        let i = Intersection::hit(xs);

        assert_eq!(i, Some(i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let shape = Shapes::Sphere(Sphere::default());

        let i1 = Intersection {
            object: shape,
            t: -2.0,
        };
        let i2 = Intersection {
            object: shape,
            t: -1.0,
        };

        let xs = vec![i2, i1];

        let i = Intersection::hit(xs);

        assert_eq!(i, None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let shape = Shapes::Sphere(Sphere::default());

        let i1 = Intersection {
            object: shape,
            t: 5.0,
        };

        let i2 = Intersection {
            object: shape,
            t: 7.0,
        };

        let i3 = Intersection {
            object: shape,
            t: -3.0,
        };

        let i4 = Intersection {
            object: shape,
            t: 2.0,
        };

        let xs = vec![i1, i2, i3, i4];

        let i = Intersection::hit(xs);

        assert_eq!(i, Some(i4));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let shape = Shapes::Sphere(Sphere::default());

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            object: shape,
            t: 4.0,
        };

        let comps = i.comps(&ray);

        assert_eq!(comps.i.t, i.t);
        assert_eq!(comps.i.object, i.object);
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let shape = Shapes::Sphere(Sphere::default());

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            object: shape,
            t: 4.0,
        };

        let comps = i.comps(&ray);

        assert!(!comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let shape = Shapes::Sphere(Sphere::default());

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            object: shape,
            t: 1.0,
        };

        let comps = i.comps(&ray);

        assert!(comps.inside);
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let shape = Shapes::Sphere(Sphere(Figure {
            transform: Matrix::translation(0.0, 0.0, 1.0),
            ..Default::default()
        }));

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            object: shape,
            t: 5.0,
        };

        let comps = i.comps(&ray);

        assert!(comps.over_point.0.z < -crate::float::EPSILON / 2.0);
        assert!(comps.point.0.z > -crate::float::EPSILON);
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = Shapes::Plane(Plane::default());

        let ray = Ray {
            origin: Point::new(0.0, 1.0, -1.0),
            direction: Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        };

        let i = Intersection {
            object: shape,
            t: 2_f64.sqrt(),
        };

        let comps = i.comps(&ray);

        assert_eq!(
            comps.reflectv,
            Vector::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0)
        );
    }
}
