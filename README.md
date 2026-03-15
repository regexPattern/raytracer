# Ray Tracer

Stochastic ray tracer based on [The Ray Tracer Challenge](http://raytracerchallenge.com/) book by Jamis Buck.

![image](https://github-production-user-asset-6210df.s3.amazonaws.com/47466248/563716784-bcbd08bb-956b-422e-af20-e4dd4a74ea64.webp?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20260315%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20260315T133836Z&X-Amz-Expires=300&X-Amz-Signature=d1bed2966b68f6dc04814c598a258434984e7428f48cfb3801f8f943fa7c96e6&X-Amz-SignedHeaders=host)

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

![image](https://github-production-user-asset-6210df.s3.amazonaws.com/47466248/563717220-922e961f-c882-42cf-bf1d-3c683a38f61c.webp?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20260315%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20260315T133925Z&X-Amz-Expires=300&X-Amz-Signature=01e69b81bd9cf7b9e88a29709b4b0be461b2945248233b6896c2704849303440&X-Amz-SignedHeaders=host)

![image](https://github-production-user-asset-6210df.s3.amazonaws.com/47466248/563717216-a5a1cd16-3e5c-4768-b603-8d390548480c.webp?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20260315%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20260315T133953Z&X-Amz-Expires=300&X-Amz-Signature=777eb170bf3cbff82004507323579bf78b3649965e31b36fd8c88b54d6477c2a&X-Amz-SignedHeaders=host)

![image](https://github-production-user-asset-6210df.s3.amazonaws.com/47466248/563717217-9d7ba09a-d1f9-4f60-b8ef-d394feb3bd44.webp?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20260315%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20260315T134014Z&X-Amz-Expires=300&X-Amz-Signature=b01f651db9c76aa8b4d8e7697d2ad2663b7a6a44109b30c8d566a3079e566b7a&X-Amz-SignedHeaders=host)

![image](https://github-production-user-asset-6210df.s3.amazonaws.com/47466248/563717218-79357c10-1e71-4b69-898d-d57f36461dea.webp?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20260315%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20260315T134034Z&X-Amz-Expires=300&X-Amz-Signature=d3b37c9c597bf6298b118c2f7f421511acf227dfe9018cf03a46b3227ddfed5c&X-Amz-SignedHeaders=host)

![image](https://github-production-user-asset-6210df.s3.amazonaws.com/47466248/563717219-8afd4b67-bd6a-4289-bde0-85f1590c014b.webp?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20260315%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20260315T134048Z&X-Amz-Expires=300&X-Amz-Signature=299f100fe62701eee32eba2709971a961eb5015a10c9853e4ee46636c1243467&X-Amz-SignedHeaders=host)
