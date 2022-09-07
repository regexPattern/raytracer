use crate::intersection::{ComputedIntersection, Intersection};
use crate::light::PointLight;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::{Intersectable, Shape, Sphere};
use crate::transformation;
use crate::tuple::{Color, Point};

pub struct World {
    pub objects: Vec<Shape>,
    pub light: PointLight,
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white());

        let s1 = Shape::Sphere(Sphere {
            material: Material {
                color: Color::new(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::default()
            },
            ..Sphere::default()
        });

        let s2 = Shape::Sphere(Sphere {
            transform: transformation::scaling(0.5, 0.5, 0.5),
            ..Sphere::default()
        });

        Self {
            objects: vec![s1, s2],
            light,
        }
    }
}

impl World {
    pub fn new(objects: Vec<Shape>, light: PointLight) -> Self {
        Self { objects, light }
    }

    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = self
            .objects
            .iter()
            .flat_map(|object| object.intersect(ray))
            .collect();

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersections
    }

    fn shade_hit(&self, comps: &ComputedIntersection) -> Color {
        let shadowed = self.is_shadowed(comps.over_point);
        comps.intersection.object.material().lighting(
            self.light,
            comps.point,
            comps.eyev,
            comps.normalv,
            shadowed,
        )
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let xs = self.intersect(ray);
        match Intersection::hit(xs) {
            Some(hit) => self.shade_hit(&hit.prepare_computations(ray)),
            None => Color::black(),
        }
    }

    fn is_shadowed(&self, point: Point) -> bool {
        let v = self.light.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(point, direction);
        let intersections = self.intersect(r);

        if let Some(hit) = Intersection::hit(intersections) {
            return hit.t < distance;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tuple::Vector;

    #[test]
    fn the_default_world() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white());

        let s1 = Shape::Sphere(Sphere {
            material: Material {
                color: Color::new(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::default()
            },
            ..Sphere::default()
        });

        let s2 = Shape::Sphere(Sphere {
            transform: transformation::scaling(0.5, 0.5, 0.5),
            ..Sphere::default()
        });

        let w = World::default();

        assert_eq!(w.light, light);
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let xs = w.intersect(r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = w.objects[0];
        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computations(r);
        let c = w.shade_hit(&comps);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));

        let c = w.color_at(r);

        assert_eq!(c, Color::black());
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let c = w.color_at(r);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let outer = Shape::Sphere(Sphere {
            material: Material {
                color: Color::new(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ambient: 1.0,
                ..Material::default()
            },
            ..Sphere::default()
        });

        let inner = Shape::Sphere(Sphere {
            material: Material {
                ambient: 1.0,
                ..Material::default()
            },
            transform: transformation::scaling(0.5, 0.5, 0.5),
        });

        let w = World {
            objects: vec![outer, inner],
            ..World::default()
        };

        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));

        let c = w.color_at(r);

        assert_eq!(c, inner.material().color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = Point::new(0.0, 10.0, 0.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::default();
        let p = Point::new(10.0, -10.0, 10.0);

        assert!(w.is_shadowed(p));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = Point::new(-20.0, 20.0, -20.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        let p = Point::new(-2.0, -2.0, -2.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());
        let s1 = Shape::Sphere(Sphere::default());
        let s2 = Shape::Sphere(Sphere {
            transform: transformation::translation(0.0, 0.0, 10.0),
            ..Sphere::default()
        });

        let w = World::new(vec![s1, s2], light);

        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s2 = w.objects[1];
        let i = Intersection::new(4.0, s2);

        let comps = i.prepare_computations(r);
        let c = w.shade_hit(&comps);

        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
}
