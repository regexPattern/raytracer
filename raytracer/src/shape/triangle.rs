use crate::{
    float,
    intersection::Intersection,
    material::Material,
    ray::Ray,
    tuple::{Point, Vector},
};

use super::{BaseShape, Shape};

#[derive(Debug, PartialEq)]
pub struct CollinearTriangleSidesError;

#[derive(Clone, Debug, PartialEq)]
pub struct Triangle {
    pub(crate) material: Material,
    pub(crate) v0: Point,
    pub(crate) v1: Point,
    pub(crate) v2: Point,
    e0: Vector,
    e1: Vector,
    normal: Vector,
}

impl Triangle {
    pub fn try_new(v0: Point, v1: Point, v2: Point) -> Result<Self, CollinearTriangleSidesError> {
        let e0 = v1 - v0;
        let e1 = v2 - v0;
        let normal = e1
            .cross(e0)
            .normalize()
            .map_err(|_| CollinearTriangleSidesError)?;

        Ok(Self {
            material: Default::default(),
            v0,
            v1,
            v2,
            e0,
            e1,
            normal,
        })
    }

    pub fn intersect<'a>(&self, object: &'a Shape, ray: Ray) -> Vec<Intersection<'a>> {
        let dir_cross_e1 = ray.direction.cross(self.e1);
        let det = self.e0.dot(dir_cross_e1);

        if float::approx(det.abs(), 0.0) {
            return vec![];
        }

        let f = 1.0 / det;
        let p0_to_origin = ray.origin - self.v0;
        let u = f * p0_to_origin.dot(dir_cross_e1);

        if u < 0.0 || u > 1.0 {
            return vec![];
        }

        let origin_cross_e0 = p0_to_origin.cross(self.e0);
        let v = f * ray.direction.dot(origin_cross_e0);

        if v < 0.0 || (u + v) > 1.0 {
            return vec![];
        }

        vec![Intersection {
            t: f * self.e1.dot(origin_cross_e0),
            object,
        }]
    }

    pub fn normal_at(&self, _: Point) -> Vector {
        self.normal
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
    fn constructing_a_triangle() {
        let v0 = Point::new(0.0, 1.0, 0.0);
        let v1 = Point::new(-1.0, 0.0, 0.0);
        let v2 = Point::new(1.0, 0.0, 0.0);

        let t = Triangle::try_new(v0, v1, v2).unwrap();

        assert_eq!(t.v0, v0);
        assert_eq!(t.v1, v1);
        assert_eq!(t.v2, v2);
        assert_eq!(t.e0, Vector::new(-1.0, -1.0, 0.0));
        assert_eq!(t.e1, Vector::new(1.0, -1.0, 0.0));
        assert_eq!(t.normal, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn trying_to_construct_a_triangle_with_collinear_sides() {
        let v0 = Point::new(1.0, 0.0, 0.0);
        let v1 = Point::new(2.0, 1.0, 0.0);
        let v2 = v0;

        let t = Triangle::try_new(v0, v1, v2);

        assert_eq!(t, Err(CollinearTriangleSidesError));
    }

    #[test]
    fn finding_the_normal_on_a_triangle() {
        let t = Triangle::try_new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        )
        .unwrap();

        let n0 = t.normal_at(Point::new(0.0, 0.5, 0.0));
        let n1 = t.normal_at(Point::new(-0.5, 0.75, 0.0));
        let n2 = t.normal_at(Point::new(0.5, 0.25, 0.0));

        assert_eq!(n0, t.normal);
        assert_eq!(n1, t.normal);
        assert_eq!(n2, t.normal);
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle() {
        let o = dummy_object();

        let t = Triangle::try_new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        )
        .unwrap();

        let r = Ray {
            origin: Point::new(0.0, -1.0, -2.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let xs = t.intersect(&o, r);

        assert!(xs.is_empty())
    }

    #[test]
    fn a_ray_misses_the_p0_p2_edge() {
        let o = dummy_object();

        let t = Triangle::try_new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        )
        .unwrap();

        let r = Ray {
            origin: Point::new(1.0, 1.0, -2.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = t.intersect(&o, r);

        assert!(xs.is_empty())
    }

    #[test]
    fn a_ray_misses_the_p0_p1_edge() {
        let o = dummy_object();

        let t = Triangle::try_new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        )
        .unwrap();

        let r = Ray {
            origin: Point::new(-1.0, 1.0, -2.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = t.intersect(&o, r);

        assert!(xs.is_empty())
    }

    #[test]
    fn a_ray_misses_the_p1_p2_edge() {
        let o = dummy_object();

        let t = Triangle::try_new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        )
        .unwrap();

        let r = Ray {
            origin: Point::new(0.0, -1.0, -2.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = t.intersect(&o, r);

        assert!(xs.is_empty())
    }

    #[test]
    fn a_ray_strikes_a_triangle() {
        let o = dummy_object();

        let t = Triangle::try_new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        )
        .unwrap();

        let r = Ray {
            origin: Point::new(0.0, 0.5, -2.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = t.intersect(&o, r);

        assert_eq!(xs.len(), 1);
        assert_approx!(xs[0].t, 2.0);
    }
}
