use crate::{
    float,
    intersection::Intersection,
    ray::Ray,
    tuple::{Point, Tuple, Vector},
};

use super::{BaseShape, Shape};

#[derive(Clone, Debug)]
pub struct Cylinder {
    pub base_shape: BaseShape,
    pub closed: bool,
    pub minimum: f64,
    pub maximum: f64,
}

impl PartialEq for Cylinder {
    fn eq(&self, other: &Self) -> bool {
        self.base_shape == other.base_shape
            && float::approx(self.minimum, other.minimum)
            && float::approx(self.maximum, other.maximum)
            && self.closed == other.closed
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            base_shape: Default::default(),
            minimum: std::f64::NEG_INFINITY,
            maximum: std::f64::INFINITY,
            closed: false,
        }
    }
}

impl Cylinder {
    pub fn intersect<'a>(&self, object: &'a Shape, ray: Ray) -> Vec<Intersection<'a>> {
        let mut xs = vec![];

        let a = ray.direction.0.x.powi(2) + ray.direction.0.z.powi(2);

        if float::approx(a, 0.0) {
            return self.intersect_caps(object, ray, xs);
        }

        let b = 2.0 * ray.origin.0.x * ray.direction.0.x + 2.0 * ray.origin.0.z * ray.direction.0.z;
        let c = ray.origin.0.x.powi(2) + ray.origin.0.z.powi(2) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return xs;
        }

        let t0 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);

        let (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };

        let y0 = ray.origin.0.y + t0 * ray.direction.0.y;
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(Intersection { t: t0, object });
        }

        let y1 = ray.origin.0.y + t1 * ray.direction.0.y;
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(Intersection { t: t1, object });
        }

        self.intersect_caps(object, ray, xs)
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        let Point(Tuple { x, y, z, .. }) = point;

        let distance = x.powi(2) + z.powi(2);

        if distance < 1.0 && float::ge(y, self.maximum - float::EPSILON) {
            Vector::new(0.0, 1.0, 0.0)
        } else if distance < 1.0 && float::le(y, self.minimum + float::EPSILON) {
            Vector::new(0.0, -1.0, 0.0)
        } else {
            Vector::new(x, 0.0, z)
        }
    }

    fn check_cap(ray: &Ray, t: f64) -> bool {
        let x = ray.origin.0.x + t * ray.direction.0.x;
        let z = ray.origin.0.z + t * ray.direction.0.z;

        float::le(x.powi(2) + z.powi(2), 1.0)
    }

    fn intersect_caps<'a>(
        &self,
        object: &'a Shape,
        ray: Ray,
        mut xs: Vec<Intersection<'a>>,
    ) -> Vec<Intersection<'a>> {
        if !self.closed || float::approx(ray.direction.0.y, 0.0) {
            return xs;
        }

        let t = (self.minimum - ray.origin.0.y) / ray.direction.0.y;
        if Self::check_cap(&ray, t) {
            xs.push(Intersection { t, object });
        }

        let t = (self.maximum - ray.origin.0.y) / ray.direction.0.y;
        if Self::check_cap(&ray, t) {
            xs.push(Intersection { t, object });
        }

        xs
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;

    use super::*;

    fn dummy_object() -> Shape {
        Shape::Cylinder(Default::default())
    }

    #[test]
    fn a_ray_misses_a_cylinder() {
        let c = Cylinder::default();

        assert!(c
            .intersect(
                &dummy_object(),
                Ray {
                    origin: Point::new(1.0, 0.0, 0.0),
                    direction: Vector::new(0.0, 1.0, 0.0)
                }
            )
            .is_empty());

        assert!(c
            .intersect(
                &dummy_object(),
                Ray {
                    origin: Point::new(0.0, 0.0, 0.0),
                    direction: Vector::new(0.0, 1.0, 0.0)
                }
            )
            .is_empty());

        assert!(c
            .intersect(
                &dummy_object(),
                Ray {
                    origin: Point::new(0.0, 0.0, -5.0),
                    direction: Vector::new(1.0, 1.0, 1.0)
                }
            )
            .is_empty());
    }

    #[test]
    fn a_ray_strikes_a_cylinder() {
        let o = dummy_object();
        let c = Cylinder::default();

        let xs = c.intersect(
            &o,
            Ray {
                origin: Point::new(1.0, 0.0, -5.0),
                direction: Vector::new(0.0, 0.0, 1.0),
            },
        );

        assert_approx!(xs[0].t, 5.0);
        assert_approx!(xs[1].t, 5.0);

        let xs = c.intersect(
            &o,
            Ray {
                origin: Point::new(0.0, 0.0, -5.0),
                direction: Vector::new(0.0, 0.0, 1.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);

        let xs = c.intersect(
            &o,
            Ray {
                origin: Point::new(0.5, 0.0, -5.0),
                direction: Vector::new(0.1, 1.0, 1.0).normalize().unwrap(),
            },
        );

        assert_approx!(xs[0].t, 6.80798);
        assert_approx!(xs[1].t, 7.08872);
    }

    #[test]
    fn normal_vector_on_a_cylinder() {
        let c = Cylinder::default();

        assert_eq!(
            c.normal_at(Point::new(1.0, 0.0, 0.0)),
            Vector::new(1.0, 0.0, 0.0)
        );

        assert_eq!(
            c.normal_at(Point::new(0.0, 5.0, -1.0)),
            Vector::new(0.0, 0.0, -1.0)
        );

        assert_eq!(
            c.normal_at(Point::new(0.0, -2.0, 1.0)),
            Vector::new(0.0, 0.0, 1.0)
        );

        assert_eq!(
            c.normal_at(Point::new(-1.0, 1.0, 0.0)),
            Vector::new(-1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn the_default_minimum_and_maximum_for_a_cylinder() {
        let c = Cylinder::default();

        assert_eq!(c.minimum, std::f64::NEG_INFINITY);
        assert_eq!(c.maximum, std::f64::INFINITY);
    }

    #[test]
    fn intersecting_a_ray_inside_constrained_cylinder() {
        let c = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            ..Default::default()
        };

        assert!(c
            .intersect(
                &dummy_object(),
                Ray {
                    origin: Point::new(0.0, 1.5, 0.0),
                    direction: Vector::new(0.1, 1.0, 0.0)
                }
            )
            .is_empty());
    }

    #[test]
    fn intersecting_a_tangent_ray_above_and_below_to_constrained_cylinders_caps() {
        let c = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            ..Default::default()
        };

        assert!(c
            .intersect(
                &dummy_object(),
                Ray {
                    origin: Point::new(0.0, 3.0, -5.0),
                    direction: Vector::new(0.0, 0.0, 1.0)
                }
            )
            .is_empty());

        assert!(c
            .intersect(
                &dummy_object(),
                Ray {
                    origin: Point::new(0.0, 0.0, -5.0),
                    direction: Vector::new(0.0, 0.0, 1.0)
                }
            )
            .is_empty());
    }

    #[test]
    fn intersecting_a_tangent_ray_exactly_through_a_constrained_cylinders_caps() {
        let c = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            ..Default::default()
        };

        assert!(c
            .intersect(
                &dummy_object(),
                Ray {
                    origin: Point::new(0.0, 2.0, -5.0),
                    direction: Vector::new(0.0, 0.0, 1.0)
                }
            )
            .is_empty());

        assert!(c
            .intersect(
                &dummy_object(),
                Ray {
                    origin: Point::new(0.0, 1.0, -5.0),
                    direction: Vector::new(0.0, 0.0, 1.0)
                }
            )
            .is_empty());
    }

    #[test]
    fn intersecting_a_constrained_cylinder_through_the_middle() {
        let c = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            ..Default::default()
        };

        assert_eq!(
            c.intersect(
                &dummy_object(),
                Ray {
                    origin: Point::new(0.0, 1.5, -2.0),
                    direction: Vector::new(0.0, 0.0, 1.0)
                }
            )
            .len(),
            2
        );
    }

    #[test]
    fn the_default_closed_value_for_a_cylinder() {
        let c = Cylinder::default();

        assert!(!c.closed);
    }

    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder() {
        let c = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            closed: true,
            ..Default::default()
        };

        assert_eq!(
            c.intersect(
                &dummy_object(),
                Ray {
                    origin: Point::new(0.0, 3.0, 0.0),
                    direction: Vector::new(0.0, -1.0, 0.0)
                }
            )
            .len(),
            2
        );

        assert_eq!(
            c.intersect(
                &dummy_object(),
                Ray {
                    origin: Point::new(0.0, 3.0, -2.0),
                    direction: Vector::new(0.0, -1.0, 2.0)
                }
            )
            .len(),
            2
        );

        assert_eq!(
            c.intersect(
                &dummy_object(),
                Ray {
                    origin: Point::new(0.0, 0.0, -2.0),
                    direction: Vector::new(0.0, 1.0, 2.0)
                }
            )
            .len(),
            2
        );
    }

    #[test]
    fn intersecting_the_border_of_the_caps_of_a_closed_cylinder() {
        let o = dummy_object();

        let c = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            closed: true,
            ..Default::default()
        };

        assert_eq!(
            c.intersect(
                &o,
                Ray {
                    origin: Point::new(0.0, 4.0, -2.0),
                    direction: Vector::new(0.0, -1.0, 1.0)
                }
            )
            .len(),
            2
        );

        assert_eq!(
            c.intersect(
                &o,
                Ray {
                    origin: Point::new(0.0, -1.0, -2.0),
                    direction: Vector::new(0.0, 1.0, 1.0)
                }
            )
            .len(),
            2
        );
    }

    #[test]
    fn the_normal_vector_on_a_cylinders_end_caps() {
        let c = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            closed: true,
            ..Default::default()
        };

        assert_eq!(
            c.normal_at(Point::new(0.0, 1.0, 0.0)),
            Vector::new(0.0, -1.0, 0.0)
        );

        assert_eq!(
            c.normal_at(Point::new(0.5, 1.0, 0.0)),
            Vector::new(0.0, -1.0, 0.0)
        );

        assert_eq!(
            c.normal_at(Point::new(0.0, 1.0, 0.5)),
            Vector::new(0.0, -1.0, 0.0)
        );

        assert_eq!(
            c.normal_at(Point::new(0.0, 2.0, 0.0)),
            Vector::new(0.0, 1.0, 0.0)
        );

        assert_eq!(
            c.normal_at(Point::new(0.5, 2.0, 0.0)),
            Vector::new(0.0, 1.0, 0.0)
        );

        assert_eq!(
            c.normal_at(Point::new(0.0, 2.0, 0.5)),
            Vector::new(0.0, 1.0, 0.0)
        );
    }
}