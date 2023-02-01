use crate::{
    color::{self, Color},
    float,
    light::Light,
    pattern::Pattern3D,
    shape::Shape,
    tuple::{Point, Vector},
};

/// Module constants.
pub mod consts {
    // You can find many indices of refraction here:
    // https://en.wikipedia.org/wiki/List_of_refractive_indices

    /// Index of refraction of vacuum.
    pub const VACUUM_INDEX_OF_REFRACTION: f64 = 1.0;

    /// Average index of refraction of air at `0` degrees celsius and `1` atm.
    pub const AIR_INDEX_OF_REFRACTION: f64 = 1.00029;

    /// Average index of refraction of water at `20` degrees celsius.
    pub const WATER_INDEX_OF_REFRACTION: f64 = 1.333;

    /// Average index of refraction of pure glass at room temperature.
    pub const GLASS_INDEX_OF_REFRACTION: f64 = 1.458;

    /// Average index of refraction of diamond at room temperature.
    pub const DIAMOND_INDEX_OF_REFRACTION: f64 = 2.417;
}

/// The material for an object.
///
/// Materials use the [Phong's reflection model](https://learnopengl.com/Lighting/Basic-Lighting)
/// to compute shading.
///
#[derive(Clone, Debug)]
pub struct Material {
    /// The pattern of the material.
    pub pattern: Pattern3D,

    /// The Phong's reflection model ambient component. It's a value between `0.0` and `1.0` that
    /// specifies the proportion of the material's effective color that gets emitted when the
    /// material is totally matte and has no shadows.
    ///
    /// The greater the value, the brighter the material gets when it's in shadows.
    ///
    pub ambient: f64,

    /// The Phong's reflection model diffuse component. It's a value between `0.0` and `1.0` that
    /// specifies how much the angle between the light ray and surface at a given point influences
    /// the shading of that material.
    ///
    /// The greater the value, the brighter the material gets when light hits it directly.
    ///
    pub diffuse: f64,

    /// The Phong's reflection model specular component. It's a value between `0.0` and `1.0` that
    /// specifies the **intensity** with which the light itself gets reflected in the material, in
    /// other words, the intensity of the "bright spot" on the material.
    ///
    /// The **size** of this "bright spot" is controled by the [shininess](Material::shininess)
    /// value.
    ///
    /// The greater the value, the brighter color of the "bright spot" is going to be.
    ///
    pub specular: f64,

    /// Controls the size of the light source reflection on the material.
    ///
    /// The lower the value, the larger the "bright spot" on the material is going to be.
    ///
    pub shininess: f64,

    /// The index of index of refraction of the material.
    pub index_of_refraction: f64,

    /// Controls the reflectivy of the material.
    ///
    /// Keep in mind that reflective materials are usually brighter, so you might what to lower the
    /// [diffuse](Material::diffuse) and [specular](Material::specular) components and use a darker
    /// pattern that what you would use with a non-reflective material.
    ///
    pub reflectivity: f64,

    /// Controls the transparency of the material.
    pub transparency: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            pattern: Pattern3D::Solid(color::consts::WHITE),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            index_of_refraction: self::consts::VACUUM_INDEX_OF_REFRACTION,
            reflectivity: 0.0,
            transparency: 0.0,
        }
    }
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

impl Material {
    /// Returns the shading color at a given point.
    ///
    /// # Arguments
    ///
    /// * `object` - Shape to which this material belongs to.
    /// * `light` - Light source used to compute the shading.
    /// * `point` - Point to shade, in world space coordinates.
    /// * `eyev` - Vector from the given point to the camera, in world space coordinates.
    /// * `normalv` - Normal vector on the object's surface at the given point, in world space coordinates.
    /// * `light_intensity` - Intensity of the light taking into account other objects in the world.
    ///
    pub(crate) fn lighting(
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

        let mut light_shade = color::consts::BLACK;

        let light_samples = match light {
            Light::Area(area_light) => area_light.samples,
            Light::Point(_) => 1,
        };

        for light_cell in light.cells() {
            let lightv = (light_cell - point)
                .normalize()
                .unwrap_or(Vector::new(0.0, 0.0, 0.0));

            let light_dot_normal = lightv.dot(normalv);

            if float::ge(light_dot_normal, 0.0) {
                let diffuse_contrib = effective_color * self.diffuse * light_dot_normal;
                light_shade = light_shade + diffuse_contrib;

                let reflectv = (-lightv).reflect(normalv);
                let reflect_dot_eye = reflectv.dot(eyev);

                if reflect_dot_eye > 0.0 {
                    let factor = reflect_dot_eye.powf(self.shininess);

                    let specular_contrib = light.intensity() * self.specular * factor;
                    light_shade = light_shade + specular_contrib;
                };
            }
        }

        ambient + (light_shade * (1.0 / light_samples as f64)) * light_intensity
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_approx,
        light::{AreaLight, AreaLightBuilder, PointLight},
        pattern::{Pattern3D, Pattern3DSpec},
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
        let material = Material::default();

        assert_eq!(material.pattern, Pattern3D::Solid(color::consts::WHITE));
        assert_approx!(material.ambient, 0.1);
        assert_approx!(material.diffuse, 0.9);
        assert_approx!(material.specular, 0.9);
        assert_approx!(material.shininess, 200.0);
        assert_approx!(material.index_of_refraction, 1.0);
        assert_approx!(material.reflectivity, 0.0);
        assert_approx!(material.transparency, 0.0);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let (object, material, position) = test_object_material_point();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::Point(PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::consts::WHITE,
        });

        let shade = material.lighting(&object, &light, position, eyev, normalv, 1.0);

        assert_eq!(
            shade,
            Color {
                red: 1.9,
                green: 1.9,
                blue: 1.9,
            }
        );
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_45_degrees() {
        let (object, material, position) = test_object_material_point();

        let eyev = Vector::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::Point(PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::consts::WHITE,
        });

        let shade = material.lighting(&object, &light, position, eyev, normalv, 1.0);

        assert_eq!(
            shade,
            Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            }
        );
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degrees() {
        let (object, material, position) = test_object_material_point();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::Point(PointLight {
            position: Point::new(0.0, 10.0, -10.0),
            intensity: color::consts::WHITE,
        });

        let shade = material.lighting(&object, &light, position, eyev, normalv, 1.0);

        assert_eq!(
            shade,
            Color {
                red: 0.7364,
                green: 0.7364,
                blue: 0.7364,
            }
        );
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let (object, material, position) = test_object_material_point();

        let eyev = Vector::new(0.0, -2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::Point(PointLight {
            position: Point::new(0.0, 10.0, -10.0),
            intensity: color::consts::WHITE,
        });

        let shade = material.lighting(&object, &light, position, eyev, normalv, 1.0);

        assert_eq!(
            shade,
            Color {
                red: 1.6364,
                green: 1.6364,
                blue: 1.6364,
            }
        );
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let (object, material, position) = test_object_material_point();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::Point(PointLight {
            position: Point::new(0.0, 0.0, 10.0),
            intensity: color::consts::WHITE,
        });

        let shade = material.lighting(&object, &light, position, eyev, normalv, 0.0);

        assert_eq!(
            shade,
            Color {
                red: 0.1,
                green: 0.1,
                blue: 0.1,
            }
        );
    }

    #[test]
    fn lighting_with_the_light_on_the_surface() {
        let (object, material, position) = test_object_material_point();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::Point(PointLight {
            position,
            intensity: color::consts::WHITE,
        });

        let shade = material.lighting(&object, &light, position, eyev, normalv, 0.0);

        assert_eq!(
            shade,
            Color {
                red: 0.1,
                green: 0.1,
                blue: 0.1
            }
        );
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let (object, material, position) = test_object_material_point();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::Point(PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::consts::WHITE,
        });

        let shade = material.lighting(&object, &light, position, eyev, normalv, 0.0);

        assert_eq!(
            shade,
            Color {
                red: 0.1,
                green: 0.1,
                blue: 0.1,
            }
        );
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let (object, _, _) = test_object_material_point();

        let material = Material {
            pattern: Pattern3D::Stripe(Pattern3DSpec::new(
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

        let shade0 = material.lighting(
            &object,
            &light,
            Point::new(0.9, 0.0, 0.0),
            eyev,
            normalv,
            0.0,
        );

        let shade1 = material.lighting(
            &object,
            &light,
            Point::new(1.1, 0.0, 0.0),
            eyev,
            normalv,
            0.0,
        );

        assert_eq!(shade0, color::consts::WHITE);
        assert_eq!(shade1, color::consts::BLACK);
    }

    #[test]
    fn lighting_uses_light_intensity_to_attenuate_color() {
        let world = test_world();

        let light = Light::Point(PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::consts::WHITE,
        });

        let object = &world.objects[0];

        let material = Material {
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.0,
            pattern: Pattern3D::Solid(color::consts::WHITE),
            ..object.as_ref().material.clone()
        };

        let point = Point::new(0.0, 0.0, -1.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);

        assert_eq!(
            material.lighting(object, &light, point, eyev, normalv, 1.0),
            color::consts::WHITE
        );

        assert_eq!(
            material.lighting(object, &light, point, eyev, normalv, 0.5),
            Color {
                red: 0.55,
                green: 0.55,
                blue: 0.55
            }
        );
        assert_eq!(
            material.lighting(object, &light, point, eyev, normalv, 0.0),
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
            horizontal_dir: horizontal_vec,
            horizontal_cells: 2,
            vertical_dir: vertical_vec,
            vertical_cells: 2,
            intensity: color::consts::WHITE,
        }));

        let object = &Shape::Sphere(Default::default());

        let material = Material {
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
            material.lighting(object, &light, point0, eyev0, normalv0, 1.0),
            Color {
                red: 0.9965,
                green: 0.9965,
                blue: 0.9965
            }
        );

        assert_eq!(
            material.lighting(object, &light, point1, eyev1, normalv1, 1.0),
            Color {
                red: 0.62318,
                green: 0.62318,
                blue: 0.62318
            }
        );
    }
}
