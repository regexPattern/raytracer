use raytracer::{
    camera::{self, consts::ImageResolution, Camera, CameraBuilder},
    color::{self, Color},
    light::{AreaLight, AreaLightBuilder, Light, PointLight},
    material::Material,
    pattern::{Pattern3D, Schema},
    scene::SceneProgress,
    shape::{
        plane::{Plane, PlaneBuilder},
        sphere::{Sphere, SphereBuilder},
        Shape,
    },
    transform::Transform,
    tuple::{Point, Vector},
    world::World,
};

const RESOLUTION: ImageResolution = camera::consts::QHD;

fn main() {
    let floor = Shape::Plane(Plane::from(PlaneBuilder {
        material: Material {
            pattern: Pattern3D::Solid(color::consts::WHITE),
            ..Default::default()
        },
        ..Default::default()
    }));

    let striped_sphere = Shape::Sphere(Sphere::from(SphereBuilder {
        material: Material {
            pattern: Pattern3D::Stripe(Schema::new(
                color::consts::WHITE,
                Color {
                    red: 0.7,
                    green: 0.4,
                    blue: 0.1,
                },
                Transform::scaling(0.25, 0.25, 0.25).unwrap()
                    * Transform::rotation_z(-std::f64::consts::FRAC_PI_4)
                    * Transform::translation(0.5, 0.0, 0.0),
            )),
            specular: 0.3,
            ..Default::default()
        },
        transform: Transform::translation(0.0, 1.0, 0.0),
    }));

    let right_light_point = Light::Point(PointLight {
        position: Point::new(10.0, 10.0, 10.0),
        intensity: color::consts::RED,
    });

    let left_light_point = Light::Point(PointLight {
        position: Point::new(-10.0, 10.0, 10.0),
        intensity: Color {
            red: 0.3216,
            green: 0.6784,
            blue: 0.03,
        },
    });

    let right_light_area = Light::Area(AreaLight::from(AreaLightBuilder {
        corner: Point::new(10.0, 10.0, 10.0),
        horizontal_vec: Vector::new(4.0, 0.0, 0.0),
        horizontal_cells: 4,
        vertical_vec: Vector::new(0.0, 4.0, 0.0),
        vertical_cells: 4,
        intensity: color::consts::RED,
    }));

    let left_light_area = Light::Area(AreaLight::from(AreaLightBuilder {
        corner: Point::new(-10.0, 10.0, 10.0),
        horizontal_vec: Vector::new(4.0, 0.0, 0.0),
        horizontal_cells: 8,
        vertical_vec: Vector::new(0.0, 4.0, 0.0),
        vertical_cells: 8,
        intensity: Color {
            red: 0.3216,
            green: 0.6784,
            blue: 0.03,
        },
    }));

    let world = World {
        objects: vec![floor, striped_sphere],
        lights: vec![left_light_area, right_light_area],
    };

    let camera = Camera::try_from(CameraBuilder {
        image_width: RESOLUTION.width,
        image_height: RESOLUTION.height,
        field_of_view: std::f64::consts::FRAC_PI_3,
        transform: Transform::view(
            Point::new(0.0, 3.0, 5.0),
            Point::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
        .unwrap(),
    })
    .unwrap();

    let image = camera.render(&world, SceneProgress::Enable).to_image();
    image.save("image.png").unwrap();
}
