# Ray Tracer

Stochastic ray tracer based on [The Ray Tracer Challenge](http://raytracerchallenge.com/) book by Jamis Buck.

![image](https://bucket.regexpattern.dev/raytracer/563716784-bcbd08bb-956b-422e-af20-e4dd4a74ea64.webp)

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

![image](https://bucket.regexpattern.dev/raytracer/563717220-922e961f-c882-42cf-bf1d-3c683a38f61c.webp)

![image](https://bucket.regexpattern.dev/raytracer/563717217-9d7ba09a-d1f9-4f60-b8ef-d394feb3bd44.webp)

![image](https://bucket.regexpattern.dev/raytracer/563717216-a5a1cd16-3e5c-4768-b603-8d390548480c.webp)

![image](https://bucket.regexpattern.dev/raytracer/563717218-79357c10-1e71-4b69-898d-d57f36461dea.webp)

![image](https://bucket.regexpattern.dev/raytracer/563717219-8afd4b67-bd6a-4289-bde0-85f1590c014b.webp)


