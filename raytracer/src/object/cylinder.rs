use crate::{
    float,
    intersections::Intersection,
    material::Material,
    ray::Ray,
    transform::Transform,
    tuple::{Point, Tuple, Vector},
};

use super::Object;

#[derive(Clone, Debug)]
pub struct Cylinder {
    pub material: Material,
    pub transform: Transform,
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

impl PartialEq for Cylinder {
    fn eq(&self, other: &Self) -> bool {
        self.material == other.material
            && self.transform == other.transform
            && float::approx(self.minimum, other.minimum)
            && float::approx(self.maximum, other.maximum)
            && self.closed == other.closed
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            material: Default::default(),
            transform: Default::default(),
            minimum: std::f64::NEG_INFINITY,
            maximum: std::f64::INFINITY,
            closed: false,
        }
    }
}

impl Cylinder {
    pub(crate) fn local_intersect<'a>(
        &self,
        object: &'a Object,
        ray: Ray,
    ) -> Vec<Intersection<'a>> {
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

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let (t1, t2) = if t1 > t2 { (t2, t1) } else { (t1, t2) };

        let y1 = ray.origin.0.y + t1 * ray.direction.0.y;
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(Intersection { t: t1, object });
        }

        let y2 = ray.origin.0.y + t2 * ray.direction.0.y;
        if self.minimum < y2 && y2 < self.maximum {
            xs.push(Intersection { t: t2, object });
        }

        self.intersect_caps(object, ray, xs)
    }

    pub(crate) fn local_normal_at(&self, point: Point) -> Vector {
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
        object: &'a Object,
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
    use super::*;

    macro_rules! assert_hits {
        ($xs:expr, $t1:expr, $t2:expr) => {{
            assert_eq!($xs.len(), 2);
            $crate::assert_approx!($xs[0].t, $t1);
            $crate::assert_approx!($xs[1].t, $t2);
        }};
    }

    fn test_cylinder_object() -> Object {
        Object::Cylinder(Default::default())
    }

    #[test]
    fn a_ray_misses_a_cylinder() {
        let o = test_cylinder_object();
        let c = Cylinder::default();

        assert_eq!(
            c.local_intersect(
                &o,
                Ray {
                    origin: Point::new(1.0, 0.0, 0.0),
                    direction: Vector::new(0.0, 1.0, 0.0)
                }
            )
            .len(),
            0
        );

        assert_eq!(
            c.local_intersect(
                &o,
                Ray {
                    origin: Point::new(0.0, 0.0, 0.0),
                    direction: Vector::new(0.0, 1.0, 0.0)
                }
            )
            .len(),
            0
        );

        assert_eq!(
            c.local_intersect(
                &o,
                Ray {
                    origin: Point::new(0.0, 0.0, -5.0),
                    direction: Vector::new(1.0, 1.0, 1.0)
                }
            )
            .len(),
            0
        );
    }

    #[test]
    fn a_ray_strikes_a_cylinder() {
        let o = test_cylinder_object();
        let c = Cylinder::default();

        assert_hits!(
            c.local_intersect(
                &o,
                Ray {
                    origin: Point::new(1.0, 0.0, -5.0),
                    direction: Vector::new(0.0, 0.0, 1.0)
                }
            ),
            5.0,
            5.0
        );

        assert_hits!(
            c.local_intersect(
                &o,
                Ray {
                    origin: Point::new(0.0, 0.0, -5.0),
                    direction: Vector::new(0.0, 0.0, 1.0)
                }
            ),
            4.0,
            6.0
        );

        assert_hits!(
            c.local_intersect(
                &o,
                Ray {
                    origin: Point::new(0.5, 0.0, -5.0),
                    direction: Vector::new(0.1, 1.0, 1.0).normalize().unwrap()
                }
            ),
            6.80798,
            7.08872
        );
    }

    #[test]
    fn normal_vector_on_a_cylinder() {
        let c = Cylinder::default();

        assert_eq!(
            c.local_normal_at(Point::new(1.0, 0.0, 0.0)),
            Vector::new(1.0, 0.0, 0.0)
        );

        assert_eq!(
            c.local_normal_at(Point::new(0.0, 5.0, -1.0)),
            Vector::new(0.0, 0.0, -1.0)
        );

        assert_eq!(
            c.local_normal_at(Point::new(0.0, -2.0, 1.0)),
            Vector::new(0.0, 0.0, 1.0)
        );

        assert_eq!(
            c.local_normal_at(Point::new(-1.0, 1.0, 0.0)),
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

        assert_eq!(
            c.local_intersect(
                &test_cylinder_object(),
                Ray {
                    origin: Point::new(0.0, 1.5, 0.0),
                    direction: Vector::new(0.1, 1.0, 0.0)
                }
            )
            .len(),
            0
        );
    }

    #[test]
    fn intersecting_a_tangent_ray_above_and_below_to_constrained_cylinders_caps() {
        let o = test_cylinder_object();

        let c = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            ..Default::default()
        };

        assert_eq!(
            c.local_intersect(
                &o,
                Ray {
                    origin: Point::new(0.0, 3.0, -5.0),
                    direction: Vector::new(0.0, 0.0, 1.0)
                }
            )
            .len(),
            0
        );

        assert_eq!(
            c.local_intersect(
                &o,
                Ray {
                    origin: Point::new(0.0, 0.0, -5.0),
                    direction: Vector::new(0.0, 0.0, 1.0)
                }
            )
            .len(),
            0
        );
    }

    #[test]
    fn intersecting_a_tangent_ray_exactly_through_a_constrained_cylinders_caps() {
        let o = test_cylinder_object();

        let c = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            ..Default::default()
        };

        assert_eq!(
            c.local_intersect(
                &o,
                Ray {
                    origin: Point::new(0.0, 2.0, -5.0),
                    direction: Vector::new(0.0, 0.0, 1.0)
                }
            )
            .len(),
            0
        );

        assert_eq!(
            c.local_intersect(
                &o,
                Ray {
                    origin: Point::new(0.0, 1.0, -5.0),
                    direction: Vector::new(0.0, 0.0, 1.0)
                }
            )
            .len(),
            0
        );
    }

    #[test]
    fn intersecting_a_constrained_cylinder_through_the_middle() {
        let c = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            ..Default::default()
        };

        assert_eq!(
            c.local_intersect(
                &test_cylinder_object(),
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
        let o = test_cylinder_object();

        let c = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            closed: true,
            ..Default::default()
        };

        assert_eq!(
            c.local_intersect(
                &o,
                Ray {
                    origin: Point::new(0.0, 3.0, 0.0),
                    direction: Vector::new(0.0, -1.0, 0.0)
                }
            )
            .len(),
            2
        );

        assert_eq!(
            c.local_intersect(
                &o,
                Ray {
                    origin: Point::new(0.0, 3.0, -2.0),
                    direction: Vector::new(0.0, -1.0, 2.0)
                }
            )
            .len(),
            2
        );

        assert_eq!(
            c.local_intersect(
                &o,
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
        let o = test_cylinder_object();

        let c = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            closed: true,
            ..Default::default()
        };

        assert_eq!(
            c.local_intersect(
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
            c.local_intersect(
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
            c.local_normal_at(Point::new(0.0, 1.0, 0.0)),
            Vector::new(0.0, -1.0, 0.0)
        );

        assert_eq!(
            c.local_normal_at(Point::new(0.5, 1.0, 0.0)),
            Vector::new(0.0, -1.0, 0.0)
        );

        assert_eq!(
            c.local_normal_at(Point::new(0.0, 1.0, 0.5)),
            Vector::new(0.0, -1.0, 0.0)
        );

        assert_eq!(
            c.local_normal_at(Point::new(0.0, 2.0, 0.0)),
            Vector::new(0.0, 1.0, 0.0)
        );

        assert_eq!(
            c.local_normal_at(Point::new(0.5, 2.0, 0.0)),
            Vector::new(0.0, 1.0, 0.0)
        );

        assert_eq!(
            c.local_normal_at(Point::new(0.0, 2.0, 0.5)),
            Vector::new(0.0, 1.0, 0.0)
        );
    }
}
