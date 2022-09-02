use crate::canvas::Color;
use crate::lighting::{Intersection, IntersectionComputation, Material, PointLight, Ray};
use crate::matrix::transformation;
use crate::shape::Sphere;
use crate::tuple::Tuple;

struct World {
    objects: Vec<Sphere>,
    light: Option<PointLight>,
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mut s1_material = Material::default();
        s1_material.color = Color::new(0.8, 1.0, 0.6);
        s1_material.diffuse = 0.7;
        s1_material.specular = 0.2;

        let s1 = Sphere::from(s1_material);

        let s2_transform = transformation::scaling(0.5, 0.5, 0.5);
        let s2 = Sphere::from(s2_transform);

        let objects = vec![s1, s2];
        let light = Some(light);

        Self { objects, light }
    }
}

impl World {
    fn new() -> Self {
        Self {
            objects: Vec::new(),
            light: None,
        }
    }

    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections = Vec::new();

        for object in &self.objects {
            let xs = object.intersect(ray);
            for intersection in xs {
                intersections.push(intersection);
            }
        }

        // TODO: Move this sorting logic to `Intersection`. This logic is also used in `lighting.rs`.
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersections
    }

    fn shade_hit(&self, comps: IntersectionComputation) -> Color {
        comps
            .object
            .material
            .lighting(self.light.unwrap(), comps.point, comps.eyev, comps.normalv)
    }

    fn color_at(&self, ray: Ray) -> Color {
        let xs = self.intersect(ray);
        match Intersection::hit(xs) {
            Some(hit) => self.shade_hit(hit.prepare_computations(ray)),
            None => Color::new(0.0, 0.0, 0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::lighting::Material;
    use crate::matrix::transformation;
    use crate::shape::Sphere;
    use crate::tuple::Tuple;

    #[test]
    fn creating_a_world() {
        let w = World::new();

        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.light, None);
    }

    #[test]
    fn the_default_world() {
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mut s1_material = Material::default();
        s1_material.color = Color::new(0.8, 1.0, 0.6);
        s1_material.diffuse = 0.7;
        s1_material.specular = 0.2;

        let s1 = Sphere::from(s1_material);

        let s2_transform = transformation::scaling(0.5, 0.5, 0.5);
        let s2 = Sphere::from(s2_transform);

        let w = World::default();

        assert_eq!(w.light, Some(light));
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
        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));

        let c = w.color_at(r);

        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
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
        let w = World::default();

        let mut outer = w.objects[0];
        outer.material.ambient = 1.0;

        let mut inner = w.objects[1];
        inner.material.ambient = 1.0;

        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));

        let c = w.color_at(r);

        assert_eq!(c, inner.material.color);
    }
}
