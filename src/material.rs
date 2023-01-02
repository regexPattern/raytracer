use crate::{
    color::{self, Color},
    light::PointLight,
    tuple::{Point, Vector},
    utils,
};

#[derive(Debug, PartialEq)]
pub struct LightOnTheSurfaceError;

#[derive(Clone, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && utils::approx(self.ambient, other.ambient)
            && utils::approx(self.diffuse, other.diffuse)
            && utils::approx(self.specular, other.specular)
            && utils::approx(self.shininess, other.shininess)
    }
}

impl Default for Material {
    fn default() -> Self {
        let color = color::consts::WHITE;
        let ambient = 0.1;
        let diffuse = 0.9;
        let specular = 0.9;
        let shininess = 200.0;

        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

impl Material {
    pub fn lighting(
        &self,
        light: &PointLight,
        point: Point,
        eyev: Vector,
        normalv: Vector,
    ) -> Result<Color, LightOnTheSurfaceError> {
        let effective_color = self.color * light.intensity;

        let lightv = (light.position - point)
            .normalize()
            .map_err(|_| LightOnTheSurfaceError)?;

        let light_dot_normal = lightv.dot(normalv);

        let ambient = effective_color * self.ambient;
        let mut diffuse = color::consts::BLACK;
        let mut specular = color::consts::BLACK;

        if utils::ge(light_dot_normal, 0.0) {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);

            if reflect_dot_eye > 0.0 {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            };
        }

        Ok(ambient + diffuse + specular)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;

    use super::*;

    fn material_and_position() -> (Material, Point) {
        (Material::default(), Point::new(0.0, 0.0, 0.0))
    }

    #[test]
    fn the_default_matrial() {
        let m = Material::default();

        assert_eq!(m.color, color::consts::WHITE);
        assert_approx!(m.ambient, 0.1);
        assert_approx!(m.diffuse, 0.9);
        assert_approx!(m.specular, 0.9);
        assert_approx!(m.shininess, 200.0);
    }

    #[test]
    fn comparing_materials() {
        let m1 = Material {
            color: color::consts::RED,
            ambient: 0.11,
            diffuse: 3.82,
            specular: 0.45,
            shininess: 14.71,
        };

        let m2 = Material {
            color: color::consts::RED,
            ambient: 0.11,
            diffuse: 3.82,
            specular: 0.45,
            shininess: 14.71,
        };

        let m3 = Material::default();

        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let (m, position) = material_and_position();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::consts::WHITE,
        };

        let result = m.lighting(&light, position, eyev, normalv);

        assert_eq!(
            result,
            Ok(Color {
                red: 1.9,
                green: 1.9,
                blue: 1.9,
            })
        );
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_45_degrees() {
        let (m, position) = material_and_position();

        let eyev = Vector::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::consts::WHITE,
        };

        let result = m.lighting(&light, position, eyev, normalv);

        assert_eq!(
            result,
            Ok(Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            })
        );
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degrees() {
        let (m, position) = material_and_position();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Point::new(0.0, 10.0, -10.0),
            intensity: color::consts::WHITE,
        };

        let result = m.lighting(&light, position, eyev, normalv);

        assert_eq!(
            result,
            Ok(Color {
                red: 0.7364,
                green: 0.7364,
                blue: 0.7364,
            })
        );
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let (m, position) = material_and_position();

        let eyev = Vector::new(0.0, -2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Point::new(0.0, 10.0, -10.0),
            intensity: color::consts::WHITE,
        };

        let result = m.lighting(&light, position, eyev, normalv);

        assert_eq!(
            result,
            Ok(Color {
                red: 1.6364,
                green: 1.6364,
                blue: 1.6364,
            })
        );
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let (m, position) = material_and_position();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Point::new(0.0, 0.0, 10.0),
            intensity: color::consts::WHITE,
        };

        let result = m.lighting(&light, position, eyev, normalv);

        assert_eq!(
            result,
            Ok(Color {
                red: 0.1,
                green: 0.1,
                blue: 0.1,
            })
        );
    }

    #[test]
    fn lighting_with_the_light_on_the_surface() {
        let (m, position) = material_and_position();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight {
            position,
            intensity: color::consts::WHITE,
        };

        let result = m.lighting(&light, position, eyev, normalv);

        assert_eq!(result, Err(LightOnTheSurfaceError));
    }
}
