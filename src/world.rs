use crate::intersection::{ComputedIntersection, Intersection};
use crate::light::PointLight;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::Sphere;
use crate::transformation;
use crate::tuple::{Color, Tuple};

pub struct World {
    objects: Vec<Sphere>,
    light: PointLight,
}

// TODO: Some of these structs don't need the `Default` trait (usage, not necesarilly
// implementation) because I'm mutating many values from the default value.
impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white());

        let s1_material = Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Material::default()
        };
        let s1 = Sphere::from(s1_material);

        let s2_transform = transformation::scaling(0.5, 0.5, 0.5);
        let s2 = Sphere::from(s2_transform);

        let objects = vec![s1, s2];
        let light = light;

        Self { objects, light }
    }
}

impl World {
    pub fn new(objects: Vec<Sphere>, light: PointLight) -> Self {
        Self { objects, light }
    }

    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = self
            .objects
            .iter()
            .map(|object| object.intersect(ray))
            .flatten()
            .collect();

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersections
    }

    fn shade_hit(&self, comps: &ComputedIntersection) -> Color {
        comps.intersection.object.material.lighting(
            self.light,
            comps.point,
            comps.eyev,
            comps.normalv,
        )
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let xs = self.intersect(ray);
        match Intersection::hit(xs) {
            Some(hit) => self.shade_hit(&hit.prepare_computations(ray)),
            None => Color::black(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_world() {
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white());

        let mut s1_material = Material::default();
        s1_material.color = Color::new(0.8, 1.0, 0.6);
        s1_material.diffuse = 0.7;
        s1_material.specular = 0.2;

        let s1 = Sphere::from(s1_material);

        let s2_transform = transformation::scaling(0.5, 0.5, 0.5);
        let s2 = Sphere::from(s2_transform);

        let w = World::default();

        assert_eq!(w.light, light);
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

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
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = w.objects[0];
        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computations(r);
        let c = w.shade_hit(&comps);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));

        let c = w.color_at(r);

        assert_eq!(c, Color::black());
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let c = w.color_at(r);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();

        let outer = &mut w.objects[0];
        outer.material.ambient = 1.0;

        let inner = &mut w.objects[1];
        inner.material.ambient = 1.0;

        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));

        let c = w.color_at(r);

        let inner = w.objects[1];
        assert_eq!(c, inner.material.color);
    }
}
