# Ray Tracer

Stochastic ray tracer based on [The Ray Tracer Challenge](http://raytracerchallenge.com/) book by Jamis Buck.

![image](https://user-images.githubusercontent.com/47466248/216463087-b68ebef4-c426-4a11-a01f-cd8fae2635d0.png)

It includes most of the features present in the book. The book only gives you the unit tests that your code must pass and some math formulas (apart from the theoretical explanation of how a raytracer works of course), so the actual implementation really reflects my own style of coding. It was also my choice to add some optimizations to this project, such as multi-threaded rendering.

## Usage

To run this project you need to have [cargo](https://github.com/rust-lang/cargo) installed. 

Refer to the [examples](https://github.com/regexPattern/raytracer/tree/main/examples) directory to find examples on how to create the scenes. You can run an example with a command such as:

```bash
cargo run --release --examples checkered_walls_metallic_sphere # runs the `examples/checkered_walls_metallic_sphere.rs` file
```

This creates an `image.png` file with the generated image.

Remember to run in **RELEASE MODE**.

### Multi-threaded rendering

This ray tracer uses the CPU to perform all the computations instead of the GPU, which usually would result in better performance due to the nature of how GPU cores work for number-crunching.

Complex scenes might require a lot of CPU power as the number of ray intersections increases. This can happen because of the following reasons:

* Increased image resolution. At least one ray is cast for every pixel in the resulting image.

* Increased number of objects in the scene. The more objects there are in a scene, the more intersections have to be checked each time a ray is casted, even if that object is nowhere near the ray.  In scenes with too many objects, adding the objects to a group and then dividing that group might allow you to take advantage of [bounding volumes hierarchy](https://en.wikipedia.org/wiki/Bounding_volume_hierarchy) and speed up your rendering time by reducing the number of unnecessary checks for intersections. You can check an example of how this is done in [this example](https://github.com/regexPattern/raytracer/blob/main/examples/multiple_glass_and_metallic_spheres.rs).

* Objects with reflective and/or refractive materials. When intersecting a material with these properties, multiple rays are cast recursively to determine the color at each point of intersection.

* Using area-lights. Multiple rays are cast towards each area-light source for every point of intersection. The exact number of rays is determined by the number of cells your area-light has.

To speed up the rendering of your scenes, you can take advantage of the multi-threaded rendering capabilities of the ray tracer, which enabled to use 8 CPU threads by default. This number can be customized by settings the `RENDER_THREADS` environment variable before running and setting its value to the number of desired threads. For example:

```bash
RENDER_THREADS=16 cargo run --release # uses 16 threads
```

### Showing rendering progress

A progress bar showing the current rendering progress can be toggled by passing the `--progress` flag when running from the command line:

```bash
cargo run --release -- --progress
```

## Showcase

![image](https://user-images.githubusercontent.com/47466248/215909726-3cce527e-0099-4a12-ba1e-9dd43e9c49ab.png)

![image](https://user-images.githubusercontent.com/47466248/215909160-94573446-b190-463f-ab7b-c5e153980720.png)

![image](https://user-images.githubusercontent.com/47466248/215910472-6fb5d0d8-6e0b-41ce-bdc8-de898fc731b2.png)

![image](https://user-images.githubusercontent.com/47466248/215910704-7cd5e01c-0906-42ee-8bfe-1e2fe19d282f.png)

![image](https://user-images.githubusercontent.com/47466248/216421620-3e8165a4-5aa9-47a8-8975-26dfaff4a338.png)
