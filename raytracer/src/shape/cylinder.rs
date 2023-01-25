use crate::{
    float,
    intersection::Intersection,
    material::Material,
    ray::Ray,
    transform::Transform,
    tuple::{Point, Tuple, Vector},
};

use super::{Bounds, Shape, ShapeProps};

#[derive(Clone, Debug)]
pub struct Cylinder {
    pub(crate) props: ShapeProps,
    pub(crate) min: f64,
    pub(crate) max: f64,
    pub(crate) closed: bool,
}

impl PartialEq for Cylinder {
    fn eq(&self, other: &Self) -> bool {
        self.props == other.props
            && float::approx(self.min, other.min)
            && float::approx(self.max, other.max)
            && self.closed == other.closed
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Self::new(
            Default::default(),
            Default::default(),
            std::f64::NEG_INFINITY,
            std::f64::INFINITY,
            false,
        )
    }
}

impl Cylinder {
    pub fn new(material: Material, transform: Transform, min: f64, max: f64, closed: bool) -> Self {
        let local_bounds = Bounds {
            min: Point::new(-1.0, min, -1.0),
            max: Point::new(1.0, max, 1.0),
        };

        Self {
            props: ShapeProps {
                material,
                transform,
                transform_inverse: transform.inverse(),
                local_bounds,
                world_bounds: local_bounds.transform(transform),
            },
            min,
            max,
            closed,
        }
    }

    pub(crate) fn intersect<'a>(&self, object: &'a Shape, ray: &Ray) -> Vec<Intersection<'a>> {
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
        if self.min < y0 && y0 < self.max {
            xs.push(Intersection {
                t: t0,
                object,
                u: None,
                v: None,
            });
        }

        let y1 = ray.origin.0.y + t1 * ray.direction.0.y;
        if self.min < y1 && y1 < self.max {
            xs.push(Intersection {
                t: t1,
                object,
                u: None,
                v: None,
            });
        }

        self.intersect_caps(object, ray, xs)
    }

    pub(crate) fn normal_at(&self, point: Point) -> Vector {
        let Point(Tuple { x, y, z, .. }) = point;

        let distance = x.powi(2) + z.powi(2);

        if distance < 1.0 && float::ge(y, self.max - float::EPSILON) {
            Vector::new(0.0, 1.0, 0.0)
        } else if distance < 1.0 && float::le(y, self.min + float::EPSILON) {
            Vector::new(0.0, -1.0, 0.0)
        } else {
            Vector::new(x, 0.0, z)
        }
    }

    fn intersect_caps<'a>(
        &self,
        object: &'a Shape,
        ray: &Ray,
        mut xs: Vec<Intersection<'a>>,
    ) -> Vec<Intersection<'a>> {
        if !self.closed || float::approx(ray.direction.0.y, 0.0) {
            return xs;
        }

        let t = (self.min - ray.origin.0.y) / ray.direction.0.y;
        if check_cap(ray, t) {
            xs.push(Intersection {
                t,
                object,
                u: None,
                v: None,
            });
        }

        let t = (self.max - ray.origin.0.y) / ray.direction.0.y;
        if check_cap(ray, t) {
            xs.push(Intersection {
                t,
                object,
                u: None,
                v: None,
            });
        }

        xs
    }
}

fn check_cap(ray: &Ray, t: f64) -> bool {
    let x = ray.origin.0.x + t * ray.direction.0.x;
    let z = ray.origin.0.z + t * ray.direction.0.z;

    float::le(x.powi(2) + z.powi(2), 1.0)
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;

    use super::*;

    #[test]
    fn a_ray_misses_a_cylinder() {
        let c = Cylinder::default();
        let o = Shape::Cylinder(Default::default());

        assert!(c
            .intersect(
                &o,
                &Ray {
                    origin: Point::new(1.0, 0.0, 0.0),
                    direction: Vector::new(0.0, 1.0, 0.0)
                }
            )
            .is_empty());

        assert!(c
            .intersect(
                &o,
                &Ray {
                    origin: Point::new(0.0, 0.0, 0.0),
                    direction: Vector::new(0.0, 1.0, 0.0)
                }
            )
            .is_empty());

        assert!(c
            .intersect(
                &o,
                &Ray {
                    origin: Point::new(0.0, 0.0, -5.0),
                    direction: Vector::new(1.0, 1.0, 1.0)
                }
            )
            .is_empty());
    }

    #[test]
    fn a_ray_strikes_a_cylinder() {
        let c = Cylinder::default();
        let o = Shape::Cylinder(Default::default());

        let xs = c.intersect(
            &o,
            &Ray {
                origin: Point::new(1.0, 0.0, -5.0),
                direction: Vector::new(0.0, 0.0, 1.0),
            },
        );

        assert_approx!(xs[0].t, 5.0);
        assert_approx!(xs[1].t, 5.0);

        let xs = c.intersect(
            &o,
            &Ray {
                origin: Point::new(0.0, 0.0, -5.0),
                direction: Vector::new(0.0, 0.0, 1.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);

        let xs = c.intersect(
            &o,
            &Ray {
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

        assert_eq!(c.min, std::f64::NEG_INFINITY);
        assert_eq!(c.max, std::f64::INFINITY);
    }

    #[test]
    fn intersecting_a_ray_inside_constrained_cylinder() {
        let c = Cylinder {
            min: 1.0,
            max: 2.0,
            ..Default::default()
        };
        let o = Shape::Cylinder(Default::default());

        assert!(c
            .intersect(
                &o,
                &Ray {
                    origin: Point::new(0.0, 1.5, 0.0),
                    direction: Vector::new(0.1, 1.0, 0.0)
                }
            )
            .is_empty());
    }

    #[test]
    fn intersecting_a_tangent_ray_above_and_below_to_constrained_cylinders_caps() {
        let c = Cylinder {
            min: 1.0,
            max: 2.0,
            ..Default::default()
        };
        let o = Shape::Cylinder(Default::default());

        assert!(c
            .intersect(
                &o,
                &Ray {
                    origin: Point::new(0.0, 3.0, -5.0),
                    direction: Vector::new(0.0, 0.0, 1.0)
                }
            )
            .is_empty());

        assert!(c
            .intersect(
                &o,
                &Ray {
                    origin: Point::new(0.0, 0.0, -5.0),
                    direction: Vector::new(0.0, 0.0, 1.0)
                }
            )
            .is_empty());
    }

    #[test]
    fn intersecting_a_tangent_ray_exactly_through_a_constrained_cylinders_caps() {
        let c = Cylinder {
            min: 1.0,
            max: 2.0,
            ..Default::default()
        };
        let o = Shape::Cylinder(Default::default());

        assert!(c
            .intersect(
                &o,
                &Ray {
                    origin: Point::new(0.0, 2.0, -5.0),
                    direction: Vector::new(0.0, 0.0, 1.0)
                }
            )
            .is_empty());

        assert!(c
            .intersect(
                &o,
                &Ray {
                    origin: Point::new(0.0, 1.0, -5.0),
                    direction: Vector::new(0.0, 0.0, 1.0)
                }
            )
            .is_empty());
    }

    #[test]
    fn intersecting_a_constrained_cylinder_through_the_middle() {
        let c = Cylinder {
            min: 1.0,
            max: 2.0,
            ..Default::default()
        };
        let o = Shape::Cylinder(Default::default());

        assert_eq!(
            c.intersect(
                &o,
                &Ray {
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
            min: 1.0,
            max: 2.0,
            closed: true,
            ..Default::default()
        };
        let o = Shape::Cylinder(Default::default());

        assert_eq!(
            c.intersect(
                &o,
                &Ray {
                    origin: Point::new(0.0, 3.0, 0.0),
                    direction: Vector::new(0.0, -1.0, 0.0)
                }
            )
            .len(),
            2
        );

        assert_eq!(
            c.intersect(
                &o,
                &Ray {
                    origin: Point::new(0.0, 3.0, -2.0),
                    direction: Vector::new(0.0, -1.0, 2.0)
                }
            )
            .len(),
            2
        );

        assert_eq!(
            c.intersect(
                &o,
                &Ray {
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
        let c = Cylinder {
            min: 1.0,
            max: 2.0,
            closed: true,
            ..Default::default()
        };
        let o = Shape::Cylinder(Default::default());

        assert_eq!(
            c.intersect(
                &o,
                &Ray {
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
                &Ray {
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
            min: 1.0,
            max: 2.0,
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

    #[test]
    fn an_unbounde_cylinder_has_a_bounding_box() {
        let c = Cylinder::default();

        let bounds = c.props.local_bounds;

        assert_eq!(bounds.max, Point::new(1.0, std::f64::INFINITY, 1.0));
        assert_eq!(bounds.min, Point::new(-1.0, std::f64::NEG_INFINITY, -1.0));
    }

    #[test]
    fn a_bounded_cylinder_has_a_bounding_box() {
        let c = Cylinder::new(Default::default(), Default::default(), -5.0, 3.0, false);

        let bounds = c.props.local_bounds;

        assert_eq!(bounds.min, Point::new(-1.0, -5.0, -1.0));
        assert_eq!(bounds.max, Point::new(1.0, 3.0, 1.0));
    }
}
