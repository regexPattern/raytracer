use crate::{
    float,
    intersections::Intersection,
    material::Material,
    ray::Ray,
    transform::Transform,
    tuple::{Point, Tuple, Vector},
};

use super::Object;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Cube {
    pub material: Material,
    pub transform: Transform,
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

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

impl Cube {
    pub fn local_intersect(object: &Object, ray: Ray) -> Vec<Intersection<'_>> {
        let (xtmin, xtmax) = check_axis(ray.origin.0.x, ray.direction.0.x);
        let (ytmin, ytmax) = check_axis(ray.origin.0.y, ray.direction.0.y);
        let (ztmin, ztmax) = check_axis(ray.origin.0.z, ray.direction.0.z);

        #[allow(clippy::unwrap_used)]
        let tmin = [xtmin, ytmin, ztmin]
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .cloned()
            .unwrap();
        let tmax = [xtmax, ytmax, ztmax]
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .cloned()
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

    pub fn local_normal_at(ray: Point) -> Vector {
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
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;

    use super::*;

    fn test_cube_object() -> Object {
        Object::Cube(Default::default())
    }

    #[test]
    fn a_ray_intersects_a_cube_from_the_x_axis() {
        let c = test_cube_object();

        let xs = Cube::local_intersect(
            &c,
            Ray {
                origin: Point::new(5.0, 0.5, 0.0),
                direction: Vector::new(-1.0, 0.0, 0.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);

        let xs = Cube::local_intersect(
            &c,
            Ray {
                origin: Point::new(-5.0, 0.5, 0.0),
                direction: Vector::new(1.0, 0.0, 0.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_cube_from_the_y_axis() {
        let c = test_cube_object();

        let xs = Cube::local_intersect(
            &c,
            Ray {
                origin: Point::new(0.5, 5.0, 0.0),
                direction: Vector::new(0.0, -1.0, 0.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);

        let xs = Cube::local_intersect(
            &c,
            Ray {
                origin: Point::new(0.5, -5.0, 0.0),
                direction: Vector::new(0.0, 1.0, 0.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_cube_from_the_z_axis() {
        let c = test_cube_object();

        let xs = Cube::local_intersect(
            &c,
            Ray {
                origin: Point::new(0.5, 0.0, 5.0),
                direction: Vector::new(0.0, 0.0, -1.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);

        let xs = Cube::local_intersect(
            &c,
            Ray {
                origin: Point::new(0.5, 0.0, -5.0),
                direction: Vector::new(0.0, 0.0, 1.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_cube_from_the_inside() {
        let c = test_cube_object();

        let xs = Cube::local_intersect(
            &c,
            Ray {
                origin: Point::new(0.0, 0.5, 0.0),
                direction: Vector::new(0.0, 0.0, 1.0),
            },
        );

        assert_approx!(xs[0].t, -1.0);
        assert_approx!(xs[1].t, 1.0);
    }

    #[test]
    fn a_ray_misses_a_cube() {
        let c = test_cube_object();

        assert_eq!(
            Cube::local_intersect(
                &c,
                Ray {
                    origin: Point::new(-2.0, 0.0, 0.0),
                    direction: Vector::new(0.2673, 0.5345, 0.8018),
                }
            )
            .len(),
            0
        );

        assert_eq!(
            Cube::local_intersect(
                &c,
                Ray {
                    origin: Point::new(0.0, -2.0, 0.0),
                    direction: Vector::new(0.8018, 0.2673, 0.5345),
                }
            )
            .len(),
            0
        );

        assert_eq!(
            Cube::local_intersect(
                &c,
                Ray {
                    origin: Point::new(0.0, 0.0, -2.0),
                    direction: Vector::new(0.5345, 0.8018, 0.2673),
                }
            )
            .len(),
            0
        );

        assert_eq!(
            Cube::local_intersect(
                &c,
                Ray {
                    origin: Point::new(2.0, 0.0, 2.0),
                    direction: Vector::new(0.0, 0.0, -1.0)
                }
            )
            .len(),
            0
        );

        assert_eq!(
            Cube::local_intersect(
                &c,
                Ray {
                    origin: Point::new(0.0, 2.0, 2.0),
                    direction: Vector::new(0.0, -1.0, 0.0)
                }
            )
            .len(),
            0
        );

        assert_eq!(
            Cube::local_intersect(
                &c,
                Ray {
                    origin: Point::new(2.0, 2.0, 0.0),
                    direction: Vector::new(-1.0, 0.0, 0.0)
                }
            )
            .len(),
            0
        );
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        assert_eq!(
            Cube::local_normal_at(Point::new(1.0, 0.5, -0.8)),
            Vector::new(1.0, 0.0, 0.0)
        );

        assert_eq!(
            Cube::local_normal_at(Point::new(-1.0, -0.2, 0.9)),
            Vector::new(-1.0, 0.0, 0.0)
        );

        assert_eq!(
            Cube::local_normal_at(Point::new(-0.4, 1.0, -0.1)),
            Vector::new(0.0, 1.0, 0.0)
        );

        assert_eq!(
            Cube::local_normal_at(Point::new(0.3, -1.0, -0.7)),
            Vector::new(0.0, -1.0, 0.0)
        );

        assert_eq!(
            Cube::local_normal_at(Point::new(-0.6, 0.3, 1.0)),
            Vector::new(0.0, 0.0, 1.0)
        );

        assert_eq!(
            Cube::local_normal_at(Point::new(0.4, 0.4, -1.0)),
            Vector::new(0.0, 0.0, -1.0)
        );
    }

    #[test]
    fn the_normal_on_the_corners_of_a_cube() {
        assert_eq!(
            Cube::local_normal_at(Point::new(1.0, 1.0, 1.0)),
            Vector::new(1.0, 0.0, 0.0)
        );

        assert_eq!(
            Cube::local_normal_at(Point::new(-1.0, -1.0, -1.0)),
            Vector::new(-1.0, 0.0, 0.0)
        );
    }
}
