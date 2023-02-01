use rand::{seq::SliceRandom, Rng};
use raytracer::{
    camera::{self, consts::ImageResolution, Camera, CameraBuilder},
    color::{self, Color},
    light::{Light, PointLight},
    material::{self, Material},
    pattern::{Pattern3D, Pattern3DSpec},
    shape::{Group, Plane, Shape, ShapeBuilder, Sphere},
    transform::Transform,
    tuple::{Point, Vector},
    world::World,
};

const RESOLUTION: ImageResolution = camera::consts::HD;

const SPHERES: i32 = 12;
const CELL_WIDTH: f64 = 2.2;

const METAL: Material = Material {
    pattern: Pattern3D::Solid(Color {
        red: 0.4863,
        green: 0.5176,
        blue: 0.5294,
    }),
    ambient: 0.1,
    diffuse: 0.9,
    index_of_refraction: material::consts::VACUUM_INDEX_OF_REFRACTION,
    reflectivity: 0.1,
    shininess: 5.0,
    specular: 0.2,
    transparency: 0.0,
};

const GLASS: Material = Material {
    pattern: Pattern3D::Solid(Color {
        red: 0.1,
        green: 0.1,
        blue: 0.1,
    }),
    ambient: 0.1,
    diffuse: 0.9,
    index_of_refraction: material::consts::GLASS_INDEX_OF_REFRACTION,
    reflectivity: 0.5,
    shininess: 400.0,
    specular: 0.9,
    transparency: 1.0,
};

fn main() {
    let mut rng = rand::thread_rng();

    let mut spheres = Group::default();
    let materials = [METAL, GLASS];

    for x in -SPHERES..SPHERES {
        for z in -SPHERES..SPHERES {
            let jitter_x = rng.gen_range(-0.2..=0.2);
            let jitter_z = rng.gen_range(-0.2..=0.2);

            let x = (f64::from(x) + 0.5 + jitter_x) * CELL_WIDTH;
            let z = (f64::from(z) + 0.5 + jitter_z) * CELL_WIDTH;

            let scaling_factor = rng.gen_range(0.25..=1.0);
            let transform = Transform::translation(x, 0.0, z)
                * Transform::scaling(scaling_factor, scaling_factor, scaling_factor).unwrap()
                * Transform::translation(0.0, 1.0, 0.0);

            let material = materials.choose(&mut rng).unwrap().clone();

            let sphere = Shape::Sphere(Sphere::from(ShapeBuilder {
                material,
                transform,
            }));
            spheres.push(sphere);
        }
    }

    let floor = Shape::Plane(Plane::from(ShapeBuilder {
        material: Material {
            pattern: Pattern3D::Checker(Pattern3DSpec::new(
                Color {
                    red: 0.9264,
                    green: 0.902,
                    blue: 0.8392,
                },
                Color {
                    red: 0.9412,
                    green: 0.9176,
                    blue: 0.8392,
                },
                Transform::scaling(0.33, 0.33, 0.33).unwrap(),
            )),
            specular: 0.1,
            ..Default::default()
        },
        ..Default::default()
    }));

    let light = Light::Point(PointLight {
        position: Point::new(-40.0, 40.0, 0.0),
        intensity: color::consts::WHITE,
    });

    spheres.divide(256);

    let world = World {
        objects: vec![floor, Shape::Group(spheres)],
        lights: vec![light],
    };

    let camera = Camera::try_from(CameraBuilder {
        width: RESOLUTION.width,
        height: RESOLUTION.height,
        field_of_view: std::f64::consts::FRAC_PI_3,
        transform: Transform::view(
            Point::new(5.0, 7.0, -10.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
        .unwrap(),
    })
    .unwrap();

    let image = camera.render(&world).to_image();
    image.save("image.png").unwrap();
}
