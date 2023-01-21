use crate::{
    float,
    intersection::Intersection,
    ray::Ray,
    tuple::{Point, Tuple, Vector},
};

use super::{BoundingBox, Shape};

pub fn intersect<'a>(
    object: &'a Shape,
    ray: &Ray,
    bounding_box: &BoundingBox,
) -> Vec<Intersection<'a>> {
    let (xtmin, xtmax) = check_axis(
        ray.origin.0.x,
        ray.direction.0.x,
        bounding_box.min.0.x,
        bounding_box.max.0.x,
    );

    let (ytmin, ytmax) = check_axis(
        ray.origin.0.y,
        ray.direction.0.y,
        bounding_box.min.0.y,
        bounding_box.max.0.y,
    );

    let (ztmin, ztmax) = check_axis(
        ray.origin.0.z,
        ray.direction.0.z,
        bounding_box.min.0.z,
        bounding_box.max.0.z,
    );

    // There's always going to be a minimum value among these.
    #[allow(clippy::unwrap_used)]
    let tmin = [xtmin, ytmin, ztmin]
        .into_iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    // Similarly there's always going to be a maximum value among these.
    #[allow(clippy::unwrap_used)]
    let tmax = [xtmax, ytmax, ztmax]
        .into_iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    if tmin > tmax {
        vec![]
    } else {
        vec![
            Intersection { t: tmin, object },
            Intersection { t: tmax, object },
        ]
    }
}

pub fn normal_at(ray: Point) -> Vector {
    let Point(Tuple { x, y, z, .. }) = ray;

    let max_coord = [x.abs(), y.abs(), z.abs()]
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .cloned()
        .unwrap();

    if float::approx(max_coord, x.abs()) {
        Vector::new(x, 0.0, 0.0)
    } else if float::approx(max_coord, y.abs()) {
        Vector::new(0.0, y, 0.0)
    } else {
        Vector::new(0.0, 0.0, z)
    }
}

fn check_axis(origin: f64, direction: f64, min: f64, max: f64) -> (f64, f64) {
    let tmin_numerator = min - origin;
    let tmax_numerator = max - origin;

    let (tmin, tmax) = if float::ge(direction.abs(), float::EPSILON) {
        (tmin_numerator / direction, tmax_numerator / direction)
    } else {
        (
            tmin_numerator * std::f64::INFINITY,
            tmax_numerator * std::f64::INFINITY,
        )
    };

    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

pub fn bounding_box() -> BoundingBox {
    BoundingBox {
        min: Point::new(-1.0, -1.0, -1.0),
        max: Point::new(1.0, 1.0, 1.0),
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;

    use super::*;

    fn dummy_object() -> Shape {
        Shape::Cube(Default::default())
    }

    #[test]
    fn a_ray_intersects_a_cube_from_the_x_axis() {
        let o = dummy_object();

        let xs = super::intersect(
            &o,
            &Ray {
                origin: Point::new(5.0, 0.5, 0.0),
                direction: Vector::new(-1.0, 0.0, 0.0),
            },
            &super::bounding_box(),
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);

        let xs = super::intersect(
            &o,
            &Ray {
                origin: Point::new(-5.0, 0.5, 0.0),
                direction: Vector::new(1.0, 0.0, 0.0),
            },
            &super::bounding_box(),
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_cube_from_the_y_axis() {
        let o = dummy_object();

        let xs = super::intersect(
            &o,
            &Ray {
                origin: Point::new(0.5, 5.0, 0.0),
                direction: Vector::new(0.0, -1.0, 0.0),
            },
            &super::bounding_box(),
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);

        let xs = super::intersect(
            &o,
            &Ray {
                origin: Point::new(0.5, -5.0, 0.0),
                direction: Vector::new(0.0, 1.0, 0.0),
            },
            &super::bounding_box(),
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_cube_from_the_z_axis() {
        let o = dummy_object();

        let xs = super::intersect(
            &o,
            &Ray {
                origin: Point::new(0.5, 0.0, 5.0),
                direction: Vector::new(0.0, 0.0, -1.0),
            },
            &super::bounding_box(),
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);

        let xs = super::intersect(
            &o,
            &Ray {
                origin: Point::new(0.5, 0.0, -5.0),
                direction: Vector::new(0.0, 0.0, 1.0),
            },
            &super::bounding_box(),
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_cube_from_the_inside() {
        let o = dummy_object();

        let r = &Ray {
            origin: Point::new(0.0, 0.5, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = super::intersect(&o, r, &super::bounding_box());

        assert_approx!(xs[0].t, -1.0);
        assert_approx!(xs[1].t, 1.0);
    }

    #[test]
    fn a_ray_misses_a_cube() {
        assert!(super::intersect(
            &dummy_object(),
            &Ray {
                origin: Point::new(-2.0, 0.0, 0.0),
                direction: Vector::new(0.2673, 0.5345, 0.8018),
            },
            &super::bounding_box()
        )
        .is_empty());

        assert!(super::intersect(
            &dummy_object(),
            &Ray {
                origin: Point::new(0.0, -2.0, 0.0),
                direction: Vector::new(0.8018, 0.2673, 0.5345),
            },
            &super::bounding_box()
        )
        .is_empty());

        assert!(super::intersect(
            &dummy_object(),
            &Ray {
                origin: Point::new(0.0, 0.0, -2.0),
                direction: Vector::new(0.5345, 0.8018, 0.2673),
            },
            &super::bounding_box()
        )
        .is_empty());

        assert!(super::intersect(
            &dummy_object(),
            &Ray {
                origin: Point::new(2.0, 0.0, 2.0),
                direction: Vector::new(0.0, 0.0, -1.0)
            },
            &super::bounding_box()
        )
        .is_empty());

        assert_eq!(
            super::intersect(
                &dummy_object(),
                &Ray {
                    origin: Point::new(0.0, 2.0, 2.0),
                    direction: Vector::new(0.0, -1.0, 0.0)
                },
                &super::bounding_box()
            )
            .len(),
            0
        );

        assert_eq!(
            super::intersect(
                &dummy_object(),
                &Ray {
                    origin: Point::new(2.0, 2.0, 0.0),
                    direction: Vector::new(-1.0, 0.0, 0.0)
                },
                &super::bounding_box()
            )
            .len(),
            0
        );
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        assert_eq!(
            super::normal_at(Point::new(1.0, 0.5, -0.8)),
            Vector::new(1.0, 0.0, 0.0)
        );

        assert_eq!(
            super::normal_at(Point::new(-1.0, -0.2, 0.9)),
            Vector::new(-1.0, 0.0, 0.0)
        );

        assert_eq!(
            super::normal_at(Point::new(-0.4, 1.0, -0.1)),
            Vector::new(0.0, 1.0, 0.0)
        );

        assert_eq!(
            super::normal_at(Point::new(0.3, -1.0, -0.7)),
            Vector::new(0.0, -1.0, 0.0)
        );

        assert_eq!(
            super::normal_at(Point::new(-0.6, 0.3, 1.0)),
            Vector::new(0.0, 0.0, 1.0)
        );

        assert_eq!(
            super::normal_at(Point::new(0.4, 0.4, -1.0)),
            Vector::new(0.0, 0.0, -1.0)
        );
    }

    #[test]
    fn the_normal_on_the_corners_of_a_cube() {
        assert_eq!(
            super::normal_at(Point::new(1.0, 1.0, 1.0)),
            Vector::new(1.0, 0.0, 0.0)
        );

        assert_eq!(
            super::normal_at(Point::new(-1.0, -1.0, -1.0)),
            Vector::new(-1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn a_cube_has_a_bounding_box() {
        let bbox = super::bounding_box();

        assert_eq!(bbox.min, Point::new(-1.0, -1.0, -1.0));
        assert_eq!(bbox.max, Point::new(1.0, 1.0, 1.0));
    }
}
