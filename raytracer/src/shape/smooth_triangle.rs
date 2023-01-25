use crate::{
    intersection::Intersection,
    ray::Ray,
    tuple::{Point, Vector},
};

use super::{Shape, Triangle};

#[derive(Clone, Debug, PartialEq)]
pub struct SmoothTriangle {
    pub(crate) triangle: Triangle,
    pub(crate) n0: Vector,
    pub(crate) n1: Vector,
    pub(crate) n2: Vector,
}

impl SmoothTriangle {
    pub(crate) fn intersect<'a>(&self, object: &'a Shape, ray: &Ray) -> Vec<Intersection<'a>> {
        self.triangle.intersect(object, ray)
    }

    pub(crate) fn normal_at(&self, _: Point, hit: &Intersection<'_>) -> Vector {
        // Smooth triangles are always ensured to have a `u` and `v` value. In fact, this is the
        // only kind of shape that has these values.
        #[allow(clippy::unwrap_used)]
        let (u, v) = (hit.u.unwrap(), hit.v.unwrap());

        self.n1 * u + self.n2 * v + self.n0 * (1.0 - u - v)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;

    use super::*;

    fn test_triangle() -> SmoothTriangle {
        SmoothTriangle {
            triangle: Triangle::try_new(
                Default::default(),
                Default::default(),
                [
                    Point::new(0.0, 1.0, 0.0),
                    Point::new(-1.0, 0.0, 0.0),
                    Point::new(1.0, 0.0, 0.0),
                ],
            )
            .unwrap(),
            n0: Vector::new(0.0, 1.0, 0.0),
            n1: Vector::new(-1.0, 0.0, 0.0),
            n2: Vector::new(1.0, 0.0, 0.0),
        }
    }

    #[test]
    fn an_intersection_with_a_smooth_triangle_stores_u_and_v() {
        let tri = test_triangle();
        let o = Shape::Sphere(Default::default());

        let r = Ray {
            origin: Point::new(-0.2, 0.3, -2.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = tri.intersect(&o, &r);

        assert_approx!(xs[0].u.unwrap(), 0.45);
        assert_approx!(xs[0].v.unwrap(), 0.25);
    }

    #[test]
    fn a_smooth_triangle_uses_u_and_v_to_interpolate_the_normal() {
        let tri = Shape::SmoothTriangle(test_triangle());

        let i = Intersection {
            t: 1.0,
            object: &tri,
            u: Some(0.45),
            v: Some(0.25),
        };

        let n = tri.normal_at(Point::new(0.0, 0.0, 0.0), &i);

        assert_eq!(n, Vector::new(-0.5547, 0.83205, 0.0));
    }

    #[test]
    fn preparing_the_normal_on_a_smooth_triangle() {
        let tri = Shape::SmoothTriangle(test_triangle());

        let i = Intersection {
            t: 1.0,
            object: &tri,
            u: Some(0.45),
            v: Some(0.25),
        };

        let r = Ray {
            origin: Point::new(-0.2, 0.3, -2.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let comps = i.prepare_computation(&r, [i]);

        assert_eq!(comps.normalv, Vector::new(-0.5547, 0.83205, 0.0));
    }
}
