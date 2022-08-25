use crate::matrix::{Matrix, Transformation};
use crate::tuple::Tuple;

#[derive(Copy, Clone, Debug)]
struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }

    fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    fn transform(&self, transformation: Matrix<4, 4>) -> Ray {
        let origin = transformation * self.origin;
        let direction = transformation * self.direction;

        Ray::new(origin, direction)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Sphere {
    transform: Matrix<4, 4>,
}

impl Sphere {
    fn new() -> Sphere {
        Sphere { transform: Matrix::default() }
    }

    fn intersect(self, r: Ray) -> Vec<Intersection> {
        let r = r.transform(self.transform.inverse());

        let sphere_to_ray = r.origin - Tuple::point(0.0, 0.0, 0.0);

        let a = r.direction.dot(r.direction);
        let b = 2.0 * r.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec::new();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![Intersection::new(t1, self), Intersection::new(t2, self)]
    }

    fn transform() {
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Intersection {
    t: f64,
    object: Sphere,
}

impl Intersection {
    fn new(t: f64, object: Sphere) -> Intersection {
        Intersection { t, object }
    }

    fn intersections(mut xs: Vec<Intersection>) -> Vec<Intersection> {
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }

    fn hit(xs: &Vec<Intersection>) -> Option<&Intersection> {
        xs.iter().find(|i| i.t.is_sign_positive())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);

        let r = Ray::new(origin, direction);

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));

        assert_eq!(r.position(0.0), Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = Intersection::intersections(vec![i1, i2]);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = Intersection::intersections(vec![i2, i1]);

        let i = Intersection::hit(&xs);

        assert_eq!(i, Some(&i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s);
        let xs = Intersection::intersections(vec![i2, i1]);

        let i = Intersection::hit(&xs);

        assert_eq!(i, Some(&i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s);
        let xs = Intersection::intersections(vec![i2, i1]);

        let i = Intersection::hit(&xs);

        assert_eq!(i, None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_non_negative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
        let xs = Intersection::intersections(vec![i1, i2, i3, i4]);

        let i = Intersection::hit(&xs);

        assert_eq!(i, Some(&i4));
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let m = Transformation::translation(3.0, 4.0, 5.0);
        let r2 = r.transform(m);

        assert_eq!(r2.origin, Tuple::point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let m = Transformation::scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(m);

        assert_eq!(r2.origin, Tuple::point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 3.0, 0.0));
    }

    #[test]
    fn a_spheres_default_transformation() {
        let s = Sphere::new();

        assert_eq!(s.transform, Matrix::from([[0.0; 4]; 4]).identity());
    }

    #[test]
    fn changing_a_spheres_transformation() {
        let mut s = Sphere::new();
        let t = Transformation::translation(2.0, 3.0, 4.0);

        s.transform = t;

        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();

        s.transform = Transformation::scaling(2.0, 2.0, 2.0);
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();

        s.transform = Transformation::translation(5.0, 0.0, 0.0);
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }
}
