mod texture;

use crate::{
    color::{self, Color},
    float,
    light::PointLight,
    shape::Shape,
    tuple::{Point, Vector},
};

pub use texture::Texture;

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub ambient: f64,
    pub diffuse: f64,
    pub reflective: f64,
    pub shininess: f64,
    pub specular: f64,
    pub texture: Texture,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: 0.1,
            diffuse: 0.9,
            reflective: 0.0,
            shininess: 200.0,
            specular: 0.9,
            texture: Texture::Color(color::WHITE),
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.texture == other.texture
            && float::approx(self.ambient, other.ambient)
            && float::approx(self.diffuse, other.diffuse)
            && float::approx(self.specular, other.specular)
            && float::approx(self.shininess, other.shininess)
    }
}

impl Material {
    pub fn lighting(
        &self,
        object: &Shape,
        light: PointLight,
        world_point: Point,
        eyev: Vector,
        normalv: Vector,
        in_shadow: bool,
    ) -> Color {
        let color = match self.texture {
            Texture::Color(c) => c,
            Texture::Pattern(p) => p.pattern_at(object, world_point),
        };

        let effective_color = color * light.intensity;

        let lightv = (light.position - world_point).normalize();

        let ambient = effective_color * self.ambient;

        let mut diffuse = color::BLACK;
        let mut specular = color::BLACK;

        let light_dot_normal = lightv.dot(normalv);

        if !in_shadow && light_dot_normal.is_sign_positive() {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflectv = (-lightv).reflect(normalv);
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
    use crate::{
        assert_approx, matrix,
        pattern::{Pattern, Scheme, Stripe},
        shape::Sphere,
    };

    use super::*;

    fn test_defaults() -> (Material, Point, Shape) {
        (
            Material::default(),
            Point::new(0.0, 0.0, 0.0),
            Shape::Sphere(Sphere::default()),
        )
    }

    #[test]
    fn the_default_material() {
        let material = Material::default();

        assert_eq!(material.texture, Texture::Color(color::WHITE));
        assert_eq!(material.ambient, 0.1);
        assert_eq!(material.diffuse, 0.9);
        assert_eq!(material.specular, 0.9);
        assert_eq!(material.shininess, 200.0);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let (material, position, object) = test_defaults();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::WHITE,
        };

        let result = material.lighting(&object, light, position, eyev, normalv, false);

        assert_eq!(
            result,
            Color {
                red: 1.9,
                green: 1.9,
                blue: 1.9
            }
        );
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45_degrees() {
        let (material, position, object) = test_defaults();

        let eyev = Vector::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::WHITE,
        };

        let result = material.lighting(&object, light, position, eyev, normalv, false);

        assert_eq!(
            result,
            Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0
            }
        );
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_light_offset_45_degrees() {
        let (material, position, object) = test_defaults();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Point::new(0.0, 10.0, -10.0),
            intensity: color::WHITE,
        };

        let result = material.lighting(&object, light, position, eyev, normalv, false);

        assert_eq!(
            result,
            Color {
                red: 0.7364,
                green: 0.7364,
                blue: 0.7364
            }
        );
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let (material, position, object) = test_defaults();

        let eyev = Vector::new(0.0, -2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Point::new(0.0, 10.0, -10.0),
            intensity: color::WHITE,
        };

        let result = material.lighting(&object, light, position, eyev, normalv, false);

        assert_eq!(
            result,
            Color {
                red: 1.6364,
                green: 1.6364,
                blue: 1.6364
            }
        );
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let (material, position, object) = test_defaults();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Point::new(0.0, 0.0, 10.0),
            intensity: color::WHITE,
        };

        let result = material.lighting(&object, light, position, eyev, normalv, false);

        assert_eq!(
            result,
            Color {
                red: 0.1,
                green: 0.1,
                blue: 0.1
            }
        );
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let (material, position, object) = test_defaults();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::WHITE,
        };
        let in_shadow = true;

        let result = material.lighting(&object, light, position, eyev, normalv, in_shadow);

        assert_eq!(
            result,
            Color {
                red: 0.1,
                green: 0.1,
                blue: 0.1
            }
        );
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let (_, _, object) = test_defaults();

        let material = Material {
            texture: Texture::Pattern(Pattern::Stripe(Stripe(Scheme {
                a: color::WHITE,
                b: color::BLACK,
                transform: matrix::IDENTITY4X4,
            }))),
            ambient: 1.0,
            diffuse: 0.0,
            specular: 0.0,
            ..Default::default()
        };

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::WHITE,
        };

        let c1 = material.lighting(
            &object,
            light,
            Point::new(0.9, 0.0, 0.0),
            eyev,
            normalv,
            false,
        );
        let c2 = material.lighting(
            &object,
            light,
            Point::new(1.1, 0.0, 0.0),
            eyev,
            normalv,
            false,
        );

        assert_eq!(c1, color::WHITE);
        assert_eq!(c2, color::BLACK);
    }

    #[test]
    fn reflectivity_for_the_default_material() {
        let material = Material::default();

        assert_approx!(material.reflective, 0.0);
    }
}
