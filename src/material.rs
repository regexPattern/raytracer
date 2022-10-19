use crate::color::{self, Color};
use crate::float;
use crate::light::PointLight;
use crate::tuple::{Point, Vector};

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub ambient: f64,
    pub color: Color,
    pub diffuse: f64,
    pub shininess: f64,
    pub specular: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: 0.1,
            color: color::WHITE,
            diffuse: 0.9,
            shininess: 200.0,
            specular: 0.9,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && float::approx(self.ambient, other.ambient)
            && float::approx(self.diffuse, other.diffuse)
            && float::approx(self.specular, other.specular)
            && float::approx(self.shininess, other.shininess)
    }
}

impl Material {
    pub fn lighting(
        &self,
        light: &PointLight,
        point: Point,
        eyev: Vector,
        normalv: Vector,
    ) -> Color {
        let effective_color = self.color * light.intensity;

        let lightv = (light.position - point).normalize();

        let ambient = effective_color * self.ambient;

        let mut diffuse = color::BLACK;
        let mut specular = color::BLACK;

        let light_dot_normal = lightv.dot(normalv);

        if light_dot_normal.is_sign_positive() {
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
    use super::*;

    fn test_defaults() -> (Material, Point) {
        (Material::default(), Point::new(0.0, 0.0, 0.0))
    }

    #[test]
    fn the_default_material() {
        let m = Material::default();

        assert_eq!(m.color, color::WHITE);
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let (m, position) = test_defaults();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::WHITE,
        };

        let result = m.lighting(&light, position, eyev, normalv);

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
        let (m, position) = test_defaults();

        let eyev = Vector::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::WHITE,
        };

        let result = m.lighting(&light, position, eyev, normalv);

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
        let (m, position) = test_defaults();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Point::new(0.0, 10.0, -10.0),
            intensity: color::WHITE,
        };

        let result = m.lighting(&light, position, eyev, normalv);

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
        let (m, position) = test_defaults();

        let eyev = Vector::new(0.0, -2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Point::new(0.0, 10.0, -10.0),
            intensity: color::WHITE,
        };

        let result = m.lighting(&light, position, eyev, normalv);

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
        let (m, position) = test_defaults();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Point::new(0.0, 0.0, 10.0),
            intensity: color::WHITE,
        };

        let result = m.lighting(&light, position, eyev, normalv);

        assert_eq!(
            result,
            Color {
                red: 0.1,
                green: 0.1,
                blue: 0.1
            }
        );
    }
}
