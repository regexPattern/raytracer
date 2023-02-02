# Ray Tracer

Stochastic ray tracer based on [The Ray Tracer Challenge](http://raytracerchallenge.com/) book by Jamis Buck.

* [API Documentation](https://regexpattern.github.io/raytracer/)

## Motivation

Initially, this was a project to learn the [Rust programming language](https://rust-lang.org), but during the process,
it became a full project on its own. It includes most of the features present in the original book, but implemented in
my way, in a new language for me, with my style of code and performance tweaks I learned on the way.

The ray tracer has gone through a lot of refactors to get to this point, mostly because of my college schedule didn't
allow me to be consistent with this project. I ended up rewriting it a couple of times before settling with an
implementation I liked.

## What is ray tracing?

Ray tracing is a method of graphics rendering that simulates the physical behavior of light over some objects (at least
[according to
Nvidia](https://developer.nvidia.com/rtx/ray-tracing#:~:text=Ray%20tracing%20is%20a%20method,to%20pioneer%20the%20technology%20since.)).
In other words, its an algorithm that allows you to create images of 3D objects with more realistic shading.

The algorithm itself works in the following way (very high level):

1. Cast a ray for every pixel of the resulting image.

2. Using each shape's mathematical formulas, determine if that ray hits each object.

3. Compute the shading (color) of that object in that particular intersection point, this is going to be the color shown
   in the image at the pixel the casted ray corresponds to.

4. If the surface struck by the ray is reflective or refractive, recursively throw another ray (whose direction its
   computed using the material properties). Recursively compute the color "contributions" of these recursive rays to the
   color of the original intersection.

5. With the color of every pixel computed, assemble the resulting image.

6. Get amazed by the result.

This particular ray tracer uses [Phong's reflection model](https://en.wikipedia.org/wiki/Phong_reflection_model) as its shading algorithm.

## Usage

To run this project you only need to have [cargo](https://github.com/rust-lang/cargo) installed. Some third-party crates
might also require you to install other dependencies, but if this is the case, you will get notified by cargo when
trying to build the project.

### Examples

These are simple examples to get you started. Refer to the
[examples](https://github.com/regexPattern/raytracer/tree/main/raytracer/examples) directory to find more complex scene
examples.

#### Creating a scene

Here's an example of how to create a scene with two light sources, and cube and a sphere laying over a plane, with
another plane simulating the sky in the background, and then render that scene and save it to a PNG image:

```rust
use raytracer::{
    camera::{Camera, CameraBuilder},
    color,
    light::{Light, PointLight},
    material::Material,
    pattern::{Pattern3D, Pattern3DSpec},
    shape::{Cube, Plane, Shape, ShapeBuilder, Sphere},
    transform::Transform,
    tuple::{Point, Vector},
    world::World,
};

fn main() {
    // Create a plane.
    let floor = Shape::Plane(Plane::from(ShapeBuilder {
        material: Material {
            pattern: Pattern3D::Solid(color::consts::DIRT),
            ..Default::default()
        },
        ..Default::default()
    }));

    // Create another plane and change it's position and rotate it.
    let sky = Shape::Plane(Plane::from(ShapeBuilder {
        material: Material {
            pattern: Pattern3D::Solid(color::consts::LIGHT_SKY_BLUE),
            specular: 0.1,
            ..Default::default()
        },
        transform: Transform::translation(0.0, 0.0, -40.0)
            * Transform::rotation_x(std::f64::consts::FRAC_PI_2),
    }));

    // Create a cube and rotate it.
    let cube = Shape::Cube(Cube::from(ShapeBuilder {
        material: Material {
            pattern: Pattern3D::Solid(color::consts::RED),
            ..Default::default()
        },
        transform: Transform::translation(0.0, 2_f64.sqrt(), 0.0)
            * Transform::rotation_y(std::f64::consts::FRAC_PI_4)
            * Transform::rotation_z(std::f64::consts::FRAC_PI_4),
    }));

    // Create a with a transformed stripe pattern.
    let sphere = Shape::Sphere(Sphere::from(ShapeBuilder {
        material: Material {
            pattern: Pattern3D::Stripe(Pattern3DSpec::new(
                color::consts::WHITE,
                color::consts::BLACK,
                Transform::rotation_z(std::f64::consts::FRAC_PI_4)
                    * Transform::scaling(0.25, 0.25, 0.25).unwrap(),
            )),
            ..Default::default()
        },
        transform: Transform::scaling(1.25, 1.25, 1.25).unwrap()
            * Transform::translation(-2.5, 1.0, 0.0),
    }));

    // Create a white point light up in the sky.
    let light = Light::Point(PointLight {
        position: Point::new(40.0, 40.0, 40.0),
        intensity: color::consts::WHITE,
    });

    // Add all of the items to a world.
    let world = World {
        objects: vec![floor, sky, cube, sphere],
        lights: vec![light],
    };

    // Create a camera to view the world, and output an HD resolution image.
    let camera = Camera::try_from(CameraBuilder {
        width: 1280,
        height: 720,
        field_of_view: std::f64::consts::FRAC_PI_3,
        transform: Transform::view(
            Point::new(-3.0, 3.0, 10.0),
            Point::new(-1.5, 1.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
        .unwrap(),
    })
    .unwrap();

    // Render the world and save to image to an PNG file.
    let image = camera.render(&world).to_image();
    image.save("my_scene.png").unwrap();
}
```

This scene looks like this:

![Ray traced image of a sphere with white and black stripes and a red rotated cube, sitting over grass with a light blue
sky behind them.](https://user-images.githubusercontent.com/47466248/216373617-9877bbc2-fbd3-4007-9c33-26a44a82e1dd.png)

#### Loading a 3D model

Loading 3D models is supported. For the time being, the only supported format for model files is [WaveFront
OBJ](https://en.wikipedia.org/wiki/Wavefront_.obj_file), but other formats may be added in the future. For OBJ models,
coloring is not supported yet.

To add a model to a scene:

1. Get an OBJ file with a model.

2. Load to contents of the file into a string (you can use `std::fs::read_to_string` for this).

3. Create an `OBJModelBuilder` to parse the file content and convert it into a `Model`.

4. Convert the model into a `Group` and optimize it if you want to.

5. Convert the group into a `Shape` and add it to your world.

For example:

```rust
use raytracer::{
    color,
    light::{Light, PointLight},
    model::{Model, OBJModelBuilder},
    shape::{Group, Shape},
    transform::Transform,
    tuple::Point,
    world::World,
};

fn main() {
    // Load the contents of the file.
    let model_spec = std::fs::read_to_string("my_model.obj").unwrap();

    // Parse the file and create a model. Also apply a transformation to it.
    let model = Model::try_from(OBJModelBuilder {
        model_spec: &model_spec,
        transform: Transform::translation(0.0, 0.5, 0.0),
    })
    .unwrap();

    // Create a group and optimize it.
    let mut group = Group::from(model);
    group.divide(64);

    let light = Light::Point(PointLight {
        position: Point::new(0.0, 7.0, 12.0),
        intensity: color::consts::WHITE,
    });

    // Convert the group to a `Shape` and add it to the world.
    let world = World {
        objects: vec![Shape::Group(group)],
        lights: vec![light],
    };
}
```

Here's an example of a render of a relatively simple model:

![A 3D model of a man in a suit with a
hat.](https://user-images.githubusercontent.com/47466248/216422859-b408fdcb-4b38-4587-a17a-58abd8217bad.png)

You can find many OBJ models online or even create your own.

### Run Configuration

> ⚠️ Remember to run in **RELEASE MODE**.  

#### Multi-threaded rendering

This ray tracer uses the CPU to perform all the computations instead of the GPU, which usually would result in better
performance due to the nature of how GPU cores work for number-crunching.

Complex scenes might require a lot of CPU power as the number of intersections increases. This can happen because of the
following reasons:

* Increased image resolution. At least one ray is cast for every pixel in the resulting image.

* Increased number of objects in the scene. The more objects there are in a scene, the more intersections have to be
  checked each time a ray is casted, even if that object is nowhere near the ray.  In scenes with too many objects,
  adding the objects to a group and then dividing that group might allow you to take advantage of bounding volumes
  [bounding](https://en.wikipedia.org/wiki/Bounding_volume_hierarchy) volume
  hierarchy](https://en.wikipedia.org/wiki/Bounding_volume_hierarchy) and speed up your rendering time by reducing the
  number of unnecessary checks for intersections. Check the [documentation](https://regexpattern.github.io/raytracer/)
  for more information about this.

* Objects with reflective and/or refractive materials. When intersecting a material with these properties, multiple rays
  are cast recursively to determine the color at each point of intersection.

* Using area-lights. Multiple rays are cast towards each area-light source for every point of intersection. The exact
  number of rays is determined by the number of cells your area-light has.

To speed up the rendering of your scenes, you can take advantage of the multi-threaded rendering capabilities of the ray
tracer, which enabled to use 8 CPU threads by default. This number can be customized by settings the `RENDER_THREADS`
environment variable before running and setting its value to the number of desired threads.

For example, using a POSIX-compliant shell:

```bash
RENDER_THREADS=16 cargo run --release # uses 16 threads
```

#### Show rendering progress

A progress bar showing the current rendering progress can be toggled by passing the `--progress` flag when running from
the command line:

```bash
cargo run --release -- --progress
```

## Showcase

* A room with white and black checkered walls, a silver metallic sphere in the middle with smaller matte red and blue
  spheres to the side. Shaded with an area-light that creates soft shadows
  ([scene](https://github.com/regexPattern/raytracer/blob/main/raytracer/examples/checkered_walls_metallic_sphere.rs)):
  ![image](https://user-images.githubusercontent.com/47466248/215909726-3cce527e-0099-4a12-ba1e-9dd43e9c49ab.png)

* Multiple randomly generated glass and metallic spheres lay on the sand. This showcases the refractive and reflective
  properties of glass and metal
  ([scene](https://github.com/regexPattern/raytracer/blob/main/raytracer/examples/multiple_glass_and_metallic_spheres.rs)):
  ![image](https://user-images.githubusercontent.com/47466248/215909160-94573446-b190-463f-ab7b-c5e153980720.png)

  > ⚠️ If you try to render this scene, beware that reflection, and specially
  refraction, require a lot of compute power because of the recursive nature of a ray tracing algorithm. This effect can
  be aggravated even more if there are lots of contiguous objects with reflective or refractive materials as well, and
  also if you are using an area-light, which required multiple rays to be casted towards the light for each point of
  intersection.

* Aerial view of a glass sphere and multiple matted spheres over a white and black checkered floor
  ([scene](https://github.com/regexPattern/raytracer/blob/main/raytracer/examples/glass_sphere_checkered_floor_aerial_view.rs)):
  ![image](https://user-images.githubusercontent.com/47466248/215910472-6fb5d0d8-6e0b-41ce-bdc8-de898fc731b2.png)

* A sphere with red stripes under the lighting of multiple area-light sources of different colors, which create red and
  green soft shadows
  ([scene](https://github.com/regexPattern/raytracer/blob/main/raytracer/examples/striped_sphere_multiple_lights.rs)):
  ![image](https://user-images.githubusercontent.com/47466248/215910704-7cd5e01c-0906-42ee-8bfe-1e2fe19d282f.png)

* 3D model of the Daft Punk helmets.
  ![image](https://user-images.githubusercontent.com/47466248/216421620-3e8165a4-5aa9-47a8-8975-26dfaff4a338.png)

## Future improvements

* **Performance:** Further optimizations can be done to speed up the file parsing, group division and image rendering
  process. I did as much as I could for someone without experience optimizing and didn't take advantage of all the tools
  that a language like Rust provides to achieve noticeable gains in performance.

* **Texture mapping:** This feature would add the capability to load textures for 3D models and also improve how
  patterns like the ring or checker pattern adapt to shape' surfaces.
  [Example](https://math.hws.edu/graphicsbook/c4/TextureDemo.png).

* **Global illumination:** Also called diffuse interreflection or color bleeding can improve the realism of images by
  making object color affected by the materials they have nearby.
  [Example](https://www.researchgate.net/profile/Christopher-Gibson-14/publication/304217750/figure/fig3/AS:380452813721635@1467718427722/Global-illumination-example-achieved-via-photon-mapping-Source.png).

* **Depth of field:** Another cool effect that allows for more photo-realistic scenes. Based on what I've researched,
  there are multiple ways to implement this effect, some are faster but less accurate than others, others the other way
  around, there are also algorithms that give you a good cost-benefit, etc.
  [Example](https://www.mattkeeter.com/projects/rayray/renders/rtiow@2x.png).

* **Web and CLI client:** This first iteration of the ray tracer allows users to consume its Rust API only, but it would
  also be possible to extend this and create an external application that consumes this API and makes it more easily
  accessible to users that don't know how to use Rust or code at all. An example of this would be a web client that
  takes a scene declared in JSON for example. I'll probably dig into this when I learn Web Assembly.

* **Rewrite it (AGAIN?):** Yes. This implementation was made following the guidelines proposed by the book, but there
  are tons of different rendering and shading techniques that I could try to implement by doing my research and building
  a ray tracer from scratch. Also, there are more efficient algorithms to compute some of the geometric shapes bound,
  for example, test for intersection or optimize area-light soft shadowing.  This could also be an opportunity to
  revisit my first Rust project after I've gained some experience, try porting it a be GPU based, and a bunch of other
  cool stuff.
