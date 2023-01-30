use crate::{
    float,
    intersection::Intersection,
    material::Material,
    ray::Ray,
    transform::Transform,
    tuple::{Point, Tuple, Vector},
};

use super::{bounding_box::BoundingBox, object::ObjectCache, Shape};

#[derive(Clone, Debug, PartialEq)]
pub struct Cube(pub(crate) ObjectCache);

#[derive(Clone, Default)]
pub struct CubeBuilder {
    pub material: Material,
    pub transform: Transform,
}

impl Default for Cube {
    fn default() -> Self {
        Self::from(CubeBuilder::default())
    }
}

impl From<CubeBuilder> for Cube {
    fn from(builder: CubeBuilder) -> Self {
        let CubeBuilder {
            material,
            transform,
        } = builder;

        let bounding_box = BoundingBox {
            min: Point::new(-1.0, -1.0, -1.0),
            max: Point::new(1.0, 1.0, 1.0),
        };

        Self(ObjectCache::new(material, transform, bounding_box))
    }
}

impl Cube {
    pub(crate) fn intersect<'a>(&self, object: &'a Shape, ray: &Ray) -> Vec<Intersection<'a>> {
        intersect_box_with_bounds(object, ray, &self.0.bounding_box)
    }

    pub(crate) fn normal_at(&self, point: Point) -> Vector {
        let Point(Tuple { x, y, z, .. }) = point;

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
}

pub fn intersect_box_with_bounds<'a>(
    object: &'a Shape,
    ray: &Ray,
    bounds: &BoundingBox,
) -> Vec<Intersection<'a>> {
    let (xtmin, xtmax) = check_axis(
        ray.origin.0.x,
        ray.direction.0.x,
        bounds.min.0.x,
        bounds.max.0.x,
    );

    let (ytmin, ytmax) = check_axis(
        ray.origin.0.y,
        ray.direction.0.y,
        bounds.min.0.y,
        bounds.max.0.y,
    );

    let (ztmin, ztmax) = check_axis(
        ray.origin.0.z,
        ray.direction.0.z,
        bounds.min.0.z,
        bounds.max.0.z,
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
            Intersection {
                t: tmin,
                object,
                u: None,
                v: None,
            },
            Intersection {
                t: tmax,
                object,
                u: None,
                v: None,
            },
        ]
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

#[cfg(test)]
mod tests {
    use crate::assert_approx;

    use super::*;

    #[test]
    fn a_ray_intersects_a_cube_from_the_x_axis() {
        let c = Cube::default();
        let o = Shape::Cube(Default::default());

        let xs = c.intersect(
            &o,
            &Ray {
                origin: Point::new(5.0, 0.5, 0.0),
                direction: Vector::new(-1.0, 0.0, 0.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);

        let xs = c.intersect(
            &o,
            &Ray {
                origin: Point::new(-5.0, 0.5, 0.0),
                direction: Vector::new(1.0, 0.0, 0.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_cube_from_the_y_axis() {
        let c = Cube::default();
        let o = Shape::Cube(Default::default());

        let xs = c.intersect(
            &o,
            &Ray {
                origin: Point::new(0.5, 5.0, 0.0),
                direction: Vector::new(0.0, -1.0, 0.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);

        let xs = c.intersect(
            &o,
            &Ray {
                origin: Point::new(0.5, -5.0, 0.0),
                direction: Vector::new(0.0, 1.0, 0.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_cube_from_the_z_axis() {
        let c = Cube::default();
        let o = Shape::Cube(Default::default());

        let xs = c.intersect(
            &o,
            &Ray {
                origin: Point::new(0.5, 0.0, 5.0),
                direction: Vector::new(0.0, 0.0, -1.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);

        let xs = c.intersect(
            &o,
            &Ray {
                origin: Point::new(0.5, 0.0, -5.0),
                direction: Vector::new(0.0, 0.0, 1.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_cube_from_the_inside() {
        let c = Cube::default();
        let o = Shape::Cube(Default::default());

        let r = &Ray {
            origin: Point::new(0.0, 0.5, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = c.intersect(&o, r);

        assert_approx!(xs[0].t, -1.0);
        assert_approx!(xs[1].t, 1.0);
    }

    #[test]
    fn a_ray_misses_a_cube() {
        let c = Cube::default();
        let o = Shape::Cube(Default::default());

        assert!(c
            .intersect(
                &o,
                &Ray {
                    origin: Point::new(-2.0, 0.0, 0.0),
                    direction: Vector::new(0.2673, 0.5345, 0.8018),
                },
            )
            .is_empty());

        assert!(c
            .intersect(
                &o,
                &Ray {
                    origin: Point::new(0.0, -2.0, 0.0),
                    direction: Vector::new(0.8018, 0.2673, 0.5345),
                },
            )
            .is_empty());

        assert!(c
            .intersect(
                &o,
                &Ray {
                    origin: Point::new(0.0, 0.0, -2.0),
                    direction: Vector::new(0.5345, 0.8018, 0.2673),
                },
            )
            .is_empty());

        assert!(c
            .intersect(
                &o,
                &Ray {
                    origin: Point::new(2.0, 0.0, 2.0),
                    direction: Vector::new(0.0, 0.0, -1.0)
                },
            )
            .is_empty());

        assert_eq!(
            c.intersect(
                &o,
                &Ray {
                    origin: Point::new(0.0, 2.0, 2.0),
                    direction: Vector::new(0.0, -1.0, 0.0)
                },
            )
            .len(),
            0
        );

        assert_eq!(
            c.intersect(
                &o,
                &Ray {
                    origin: Point::new(2.0, 2.0, 0.0),
                    direction: Vector::new(-1.0, 0.0, 0.0)
                },
            )
            .len(),
            0
        );
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        let c = Cube::default();

        assert_eq!(
            c.normal_at(Point::new(1.0, 0.5, -0.8)),
            Vector::new(1.0, 0.0, 0.0)
        );

        assert_eq!(
            c.normal_at(Point::new(-1.0, -0.2, 0.9)),
            Vector::new(-1.0, 0.0, 0.0)
        );

        assert_eq!(
            c.normal_at(Point::new(-0.4, 1.0, -0.1)),
            Vector::new(0.0, 1.0, 0.0)
        );

        assert_eq!(
            c.normal_at(Point::new(0.3, -1.0, -0.7)),
            Vector::new(0.0, -1.0, 0.0)
        );

        assert_eq!(
            c.normal_at(Point::new(-0.6, 0.3, 1.0)),
            Vector::new(0.0, 0.0, 1.0)
        );

        assert_eq!(
            c.normal_at(Point::new(0.4, 0.4, -1.0)),
            Vector::new(0.0, 0.0, -1.0)
        );
    }

    #[test]
    fn the_normal_on_the_corners_of_a_cube() {
        let c = Cube::default();

        assert_eq!(
            c.normal_at(Point::new(1.0, 1.0, 1.0)),
            Vector::new(1.0, 0.0, 0.0)
        );

        assert_eq!(
            c.normal_at(Point::new(-1.0, -1.0, -1.0)),
            Vector::new(-1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn a_cube_has_a_bounding_box() {
        let c = Cube::default();
        let bounds = c.0.bounding_box;

        assert_eq!(bounds.min, Point::new(-1.0, -1.0, -1.0));
        assert_eq!(bounds.max, Point::new(1.0, 1.0, 1.0));
    }
}
