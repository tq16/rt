# RT — Ray Tracer

A ray tracer written in Rust that renders 3D scenes into ASCII PPM (`P3`) image files.

The program sends one ray from the camera through every pixel. Each ray checks whether it hits an object in the scene. If it does, the object is shaded using its material, a point light, and shadows. If it misses, the program renders a sky background.

## Features

- Renders PPM (`P3`) images.
- Configurable output resolution.
- Sphere intersection.
- Flat plane intersection.
- Closed vertical cylinder intersection.
- Axis-aligned cube intersection.
- Object location, size, radius, height, and material colors can be changed.
- Point light with configurable intensity.
- Ambient and diffuse lighting.
- Hard shadows.
- Movable camera with configurable position, target, and field of view.
- Four prepared mandatory scenes in the `scenes/` directory.

## Requirements

- Rust and Cargo

Check that Cargo is installed:

```bash
cargo --version
```

## Running the Program

The program writes the PPM image to standard output. Redirect the output into a `.ppm` file.

General command:

```bash
cargo run -- <scene> --width <width> --height <height> > output.ppm
```

Example:

```bash
cargo run -- all --width 800 --height 600 > output.ppm
```

Open the image on Ubuntu:

```bash
xdg-open output.ppm
```

## Available Scenes

| Scene name | Description |
|---|---|
| `sphere` | One red sphere |
| `plane-cube` | A blue cube on a flat plane with lower brightness |
| `all` | A sphere, cylinder, cube, and flat plane |
| `all-alt` | The same four objects from a different camera angle |

Examples:

```bash
cargo run -- sphere --width 800 --height 600 > sphere.ppm
cargo run -- plane-cube --width 800 --height 600 > plane_cube.ppm
cargo run -- all --width 800 --height 600 > all_objects.ppm
cargo run -- all-alt --width 800 --height 600 > all_objects_alt.ppm
```

## Prepared Mandatory Images

The project includes these images:

```text
scenes/01_sphere.ppm
scenes/02_plane_cube.ppm
scenes/03_all_objects.ppm
scenes/04_all_objects_alt.ppm
```

All prepared images use `800 × 600` resolution.

## Creating Objects

Objects are added to a `Scene` using `scene.add(Box::new(...))`.

First, create a material:

```rust
let red = Material::new(
    Vec3::new(0.85, 0.15, 0.12),
    0.12,
    0.88,
);
```

`Material::new(color, ambient, diffuse)` uses:

- `color`: RGB values from `0.0` to `1.0`
- `ambient`: minimum brightness on the object
- `diffuse`: how strongly direct light affects the object

### Sphere

```rust
scene.add(Box::new(Sphere::new(
    Vec3::new(0.0, 0.0, -2.0),
    0.5,
    red,
)));
```

Arguments:

1. Center position
2. Radius
3. Material

### Flat Plane

```rust
scene.add(Box::new(Plane::new(
    Vec3::new(0.0, -0.5, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    floor_gray,
)));
```

Arguments:

1. A point on the plane
2. Plane normal direction
3. Material

The normal `(0.0, 1.0, 0.0)` makes the plane face upward like a floor.

### Cylinder

```rust
scene.add(Box::new(Cylinder::new(
    Vec3::new(-0.85, -0.05, -1.45),
    0.30,
    0.90,
    green,
)));
```

Arguments:

1. Center position
2. Radius
3. Height
4. Material

The cylinder is vertical and includes a top cap, bottom cap, and curved side.

### Cube

```rust
scene.add(Box::new(Cube::new(
    Vec3::new(1.15, -0.10, -1.90),
    0.80,
    blue,
)));
```

Arguments:

1. Center position
2. Equal size for width, height, and depth
3. Material

The cube is axis-aligned, meaning it does not rotate.

## Changing Brightness

The scene uses a point light:

```rust
let light = PointLight::new(
    Vec3::new(-2.0, 2.5, 0.0),
    1.0,
);
```

Arguments:

1. Light position
2. Light intensity

Use a higher intensity for a brighter scene:

```rust
let light = PointLight::new(
    Vec3::new(-2.0, 2.5, 0.0),
    1.5,
);
```

Use a lower intensity for a darker scene:

```rust
let light = PointLight::new(
    Vec3::new(-2.0, 2.5, 0.0),
    0.45,
);
```

Brightness is also affected by the material's `ambient` and `diffuse` values.

## Changing Camera Position and Angle

The camera is created with:

```rust
let camera = Camera::new(
    Vec3::new(2.8, 1.6, 2.8),
    Vec3::new(0.0, -0.1, -1.4),
    Vec3::new(0.0, 1.0, 0.0),
    aspect_ratio,
    45.0,
);
```

Arguments:

1. `look_from`: camera position
2. `look_at`: point where the camera looks
3. `world_up`: upward direction, normally `(0.0, 1.0, 0.0)`
4. `aspect_ratio`: image width divided by image height
5. `field_of_view`: camera zoom in degrees

Move `look_from` to move the camera. Change `look_at` to point the camera in another direction. A smaller field of view zooms in, while a larger field of view zooms out.

## Rendering Process

For each pixel, the ray tracer:

1. Gets a ray from the camera.
2. Finds the closest object touched by that ray.
3. Calculates the surface normal at the hit point.
4. Sends a shadow ray toward the point light.
5. Applies ambient and diffuse lighting.
6. Writes the final RGB color into the PPM image.

## Project Structure

```text
src/
├── camera.rs        Camera position and ray generation
├── image.rs         PPM color output
├── light.rs         Point light definition
├── material.rs      Object material and brightness values
├── ray.rs           Ray origin, direction, and position
├── scene.rs         Stores all scene objects
├── math/
│   └── vec3.rs      3D vector mathematics
└── objects/
    ├── sphere.rs    Sphere intersection
    ├── plane.rs     Plane intersection
    ├── cylinder.rs  Cylinder intersection
    └── cube.rs      Cube intersection
```