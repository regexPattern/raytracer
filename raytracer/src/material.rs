use crate::{
    color::{self, Color},
    float,
    light::Light,
    pattern::Pattern3D,
    shape::Shape,
    tuple::{Point, Vector},
};

pub mod consts {
    pub const VACUUM_INDEX_OF_REFRACTION: f64 = 1.0;
    pub const AIR_INDEX_OF_REFRACTION: f64 = 1.00029;
    pub const WATER_INDEX_OF_REFRACTION: f64 = 1.333;
    pub const GLASS_INDEX_OF_REFRACTION: f64 = 1.52;
}

#[derive(Clone, Debug)]
pub struct Material {
    pub ambient: f64,
    pub diffuse: f64,
    pub index_of_refraction: f64,
    pub pattern: Pattern3D,
    pub reflectivity: f64,
    pub shininess: f64,
    pub specular: f64,
    pub transparency: f64,
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
            && float::approx(self.ambient, other.ambient)
            && float::approx(self.diffuse, other.diffuse)
            && float::approx(self.index_of_refraction, other.index_of_refraction)
            && float::approx(self.reflectivity, other.reflectivity)
            && float::approx(self.shininess, other.shininess)
            && float::approx(self.specular, other.specular)
            && float::approx(self.transparency, other.transparency)
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: 0.1,
            diffuse: 0.9,
            index_of_refraction: self::consts::VACUUM_INDEX_OF_REFRACTION,
            pattern: Pattern3D::Solid(color::consts::WHITE),
            reflectivity: 0.0,
            shininess: 200.0,
            specular: 0.9,
            transparency: 0.0,
        }
    }
}

impl Material {
    pub fn lighting(
        &self,
        object: &Shape,
        light: &Light,
        point: Point,
        eyev: Vector,
        normalv: Vector,
        light_intensity: f64,
    ) -> Color {
        let effective_color = self.pattern.color_at_object(object, point) * light.intensity();

        let ambient = effective_color * self.ambient;

        let mut sum = color::consts::BLACK;

        let light_samples = match light {
            Light::Area(area_light) => area_light.samples,
            Light::Point(_) => 1,
        };

        for sample in light.samples() {
            let lightv = (sample - point)
                .normalize()
                .unwrap_or(Vector::new(0.0, 0.0, 0.0));

            let light_dot_normal = lightv.dot(normalv);

            if float::ge(light_dot_normal, 0.0) {
                let diffuse_contrib = effective_color * self.diffuse * light_dot_normal;
                sum = sum + diffuse_contrib;

                let reflectv = (-lightv).reflect(normalv);
                let reflect_dot_eye = reflectv.dot(eyev);

                if reflect_dot_eye > 0.0 {
                    let factor = reflect_dot_eye.powf(self.shininess);

                    let specular_contrib = light.intensity() * self.specular * factor;
                    sum = sum + specular_contrib;
                };
            }
        }

        ambient + (sum * (1.0 / light_samples as f64)) * light_intensity
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_approx,
        light::{AreaLight, AreaLightBuilder, PointLight},
        pattern::{Pattern3D, Schema},
        world::test_world,
    };

    use super::*;

    fn test_object_material_point() -> (Shape, Material, Point) {
        (
            Shape::Sphere(Default::default()),
            Default::default(),
            Point::new(0.0, 0.0, 0.0),
        )
    }

    #[test]
    fn the_default_matrial() {
        let m = Material::default();

        assert_eq!(m.pattern, Pattern3D::Solid(color::consts::WHITE));
        assert_approx!(m.ambient, 0.1);
        assert_approx!(m.diffuse, 0.9);
        assert_approx!(m.specular, 0.9);
        assert_approx!(m.shininess, 200.0);
        assert_approx!(m.reflectivity, 0.0);
        assert_approx!(m.transparency, 0.0);
        assert_approx!(m.index_of_refraction, 1.0);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let (object, m, position) = test_object_material_point();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::Point(PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::consts::WHITE,
        });

        let result = m.lighting(&object, &light, position, eyev, normalv, 1.0);

        assert_eq!(
            result,
            Color {
                red: 1.9,
                green: 1.9,
                blue: 1.9,
            }
        );
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_45_degrees() {
        let (object, m, position) = test_object_material_point();

        let eyev = Vector::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::Point(PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::consts::WHITE,
        });

        let result = m.lighting(&object, &light, position, eyev, normalv, 1.0);

        assert_eq!(
            result,
            Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            }
        );
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degrees() {
        let (object, m, position) = test_object_material_point();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::Point(PointLight {
            position: Point::new(0.0, 10.0, -10.0),
            intensity: color::consts::WHITE,
        });

        let result = m.lighting(&object, &light, position, eyev, normalv, 1.0);

        assert_eq!(
            result,
            Color {
                red: 0.7364,
                green: 0.7364,
                blue: 0.7364,
            }
        );
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let (object, m, position) = test_object_material_point();

        let eyev = Vector::new(0.0, -2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::Point(PointLight {
            position: Point::new(0.0, 10.0, -10.0),
            intensity: color::consts::WHITE,
        });

        let result = m.lighting(&object, &light, position, eyev, normalv, 1.0);

        assert_eq!(
            result,
            Color {
                red: 1.6364,
                green: 1.6364,
                blue: 1.6364,
            }
        );
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let (object, m, position) = test_object_material_point();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::Point(PointLight {
            position: Point::new(0.0, 0.0, 10.0),
            intensity: color::consts::WHITE,
        });

        let result = m.lighting(&object, &light, position, eyev, normalv, 0.0);

        assert_eq!(
            result,
            Color {
                red: 0.1,
                green: 0.1,
                blue: 0.1,
            }
        );
    }

    #[test]
    fn lighting_with_the_light_on_the_surface() {
        let (object, m, position) = test_object_material_point();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::Point(PointLight {
            position,
            intensity: color::consts::WHITE,
        });

        let result = m.lighting(&object, &light, position, eyev, normalv, 0.0);

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
        let (object, m, position) = test_object_material_point();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::Point(PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::consts::WHITE,
        });

        let result = m.lighting(&object, &light, position, eyev, normalv, 0.0);

        assert_eq!(
            result,
            Color {
                red: 0.1,
                green: 0.1,
                blue: 0.1,
            }
        );
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let (o, _, _) = test_object_material_point();

        let m = Material {
            pattern: Pattern3D::Stripe(Schema::new(
                color::consts::WHITE,
                color::consts::BLACK,
                Default::default(),
            )),
            ambient: 1.0,
            diffuse: 0.0,
            specular: 0.0,
            ..Default::default()
        };

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::Point(PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::consts::WHITE,
        });

        let c0 = m.lighting(&o, &light, Point::new(0.9, 0.0, 0.0), eyev, normalv, 0.0);
        let c1 = m.lighting(&o, &light, Point::new(1.1, 0.0, 0.0), eyev, normalv, 0.0);

        assert_eq!(c0, color::consts::WHITE);
        assert_eq!(c1, color::consts::BLACK);
    }

    #[test]
    fn lighting_uses_light_intensity_to_attenuate_color() {
        let w = test_world();

        let light = Light::Point(PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::consts::WHITE,
        });

        let shape = &w.objects[0];
        let m = Material {
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.0,
            pattern: Pattern3D::Solid(color::consts::WHITE),
            ..shape.as_ref().material.clone()
        };

        let point = Point::new(0.0, 0.0, -1.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);

        assert_eq!(
            m.lighting(shape, &light, point, eyev, normalv, 1.0),
            color::consts::WHITE
        );
        assert_eq!(
            m.lighting(shape, &light, point, eyev, normalv, 0.5),
            Color {
                red: 0.55,
                green: 0.55,
                blue: 0.55
            }
        );
        assert_eq!(
            m.lighting(shape, &light, point, eyev, normalv, 0.0),
            Color {
                red: 0.1,
                green: 0.1,
                blue: 0.1
            }
        );
    }

    #[test]
    fn lighting_samples_the_area_light() {
        let corner = Point::new(-0.5, -0.5, -5.0);

        let horizontal_vec = Vector::new(1.0, 0.0, 0.0);
        let vertical_vec = Vector::new(0.0, 1.0, 0.0);

        let light = Light::Area(AreaLight::from(AreaLightBuilder {
            corner,
            horizontal_vec,
            horizontal_cells: 2,
            vertical_vec,
            vertical_cells: 2,
            intensity: color::consts::WHITE,
        }));

        let shape = &Shape::Sphere(Default::default());
        let m = Material {
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.0,
            pattern: Pattern3D::Solid(color::consts::WHITE),
            ..Default::default()
        };

        let eye = Point::new(0.0, 0.0, -5.0);

        let point0 = Point::new(0.0, 0.0, -1.0);
        let eyev0 = (eye - point0).normalize().unwrap();
        let normalv0 = Vector::new(point0.0.x, point0.0.y, point0.0.z);

        let point1 = Point::new(0.0, 0.7071, -0.7071);
        let eyev1 = (eye - point1).normalize().unwrap();
        let normalv1 = Vector::new(point1.0.x, point1.0.y, point1.0.z);

        assert_eq!(
            m.lighting(shape, &light, point0, eyev0, normalv0, 1.0),
            Color {
                red: 0.9965,
                green: 0.9965,
                blue: 0.9965
            }
        );

        assert_eq!(
            m.lighting(shape, &light, point1, eyev1, normalv1, 1.0),
            Color {
                red: 0.62318,
                green: 0.62318,
                blue: 0.62318
            }
        );
    }
}
