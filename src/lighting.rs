use crate::canvas::Color;
use crate::matrix::Matrix;
use crate::shape::Sphere;
use crate::tuple::Tuple;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }

    pub fn position(self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn transform(self, m: Matrix<4, 4>) -> Self {
        let origin = m * self.origin;
        let direction = m * self.direction;
        Self::new(origin, direction)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub object: Sphere,
}

#[derive(Copy, Clone, Debug)]
pub struct IntersectionComputation {
    t: f64,
    pub object: Sphere,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    inside: bool,
}

impl Intersection {
    pub fn new(t: f64, object: Sphere) -> Self {
        Self { t, object }
    }

    pub fn hit(mut xs: Vec<Intersection>) -> Option<Intersection> {
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs.into_iter().find(|i| i.t.is_sign_positive())
    }

    pub fn prepare_computations(self, ray: Ray) -> IntersectionComputation {
        let t = self.t;
        let object = self.object;
        let point = ray.position(t);
        let eyev = -ray.direction;
        let mut normalv = object.normal_at(point);

        let inside = if normalv.dot(eyev) < 0.0 {
            normalv = -normalv;
            true
        } else {
            false
        };

        IntersectionComputation {
            t,
            object,
            point,
            eyev,
            normalv,
            inside,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PointLight {
    position: Tuple,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl Material {
    pub fn lighting(self, light: PointLight, point: Tuple, eyev: Tuple, normalv: Tuple) -> Color {
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - point).normalize();

        let ambient = effective_color * self.ambient;

        let light_dot_normal = lightv.dot(normalv);

        let mut diffuse = Color::black();
        let mut specular = Color::black();

        if light_dot_normal >= 0.0 {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflectv = -lightv.reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);

            if reflect_dot_eye > 0.0 {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::matrix::transformation;
    use crate::shape::Sphere;

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
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::default();
        let i = Intersection::new(3.5, s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);

        let xs = vec![i1, i2];

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = vec![i2, i1];

        let i = Intersection::hit(xs);

        assert_eq!(i, Some(i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s);
        let xs = vec![i2, i1];

        let i = Intersection::hit(xs);

        assert_eq!(i, Some(i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s);
        let xs = vec![i2, i1];

        let i = Intersection::hit(xs);

        assert_eq!(i, None)
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::default();
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
        let xs = vec![i1, i2, i3, i4];

        let i = Intersection::hit(xs);

        assert_eq!(i, Some(i4));
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let m = transformation::translation(3.0, 4.0, 5.0);

        let r2 = r.transform(m);

        assert_eq!(r2.origin, Tuple::point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let m = transformation::scaling(2.0, 3.0, 4.0);

        let r2 = r.transform(m);

        assert_eq!(r2.origin, Tuple::point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 3.0, 0.0));
    }

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Color::white();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let light = PointLight::new(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }

    #[test]
    fn the_default_material() {
        let m = Material::default();

        assert_eq!(m.color, Color::white());
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::white());

        let result = m.lighting(light, position, eyev, normalv);

        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_45_degrees() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eyev = Tuple::vector(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::white());

        let result = m.lighting(light, position, eyev, normalv);

        assert_eq!(result, Color::white());
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_light_offset_45_degrees() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::white());

        let result = m.lighting(light, position, eyev, normalv);

        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_the_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eyev = Tuple::vector(0.0, -2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::white());

        let result = m.lighting(light, position, eyev, normalv);

        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_light_behind_the_surface() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Color::white());

        let result = m.lighting(light, position, eyev, normalv);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    // TODO: Move structs to their own files.
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computations(r);

        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computations(r);

        assert!(!comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(1.0, shape);

        let comps = i.prepare_computations(r);

        assert_eq!(comps.point, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }
}
