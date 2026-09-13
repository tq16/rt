mod camera;
mod image;
mod light;
mod material;
mod math;
mod objects;
mod ray;
mod scene;

use std::env;

use camera::Camera;
use image::write_color;
use light::PointLight;
use material::Material;
use math::vec3::Vec3;
use objects::cube::Cube;
use objects::cylinder::Cylinder;
use objects::hittable::Hittable;
use objects::plane::Plane;
use objects::sphere::Sphere;
use ray::Ray;
use scene::Scene;

fn ray_color(ray: Ray, scene: &Scene, light: &PointLight) -> Vec3 {
    if let Some(hit) = scene.hit(ray, 0.001, f64::INFINITY) {
        let to_light = light.position - hit.point;
        let light_distance = to_light.length();
        let light_direction = to_light.normalize();

        let shadow_ray = Ray::new(hit.point + hit.normal * 0.001, light_direction);

        let is_in_shadow = scene.hit(shadow_ray, 0.001, light_distance).is_some();

        let ambient_light = hit.material.ambient;

        let diffuse_light = if is_in_shadow {
            0.0
        } else {
            let light_facing_surface = hit.normal.dot(light_direction).max(0.0);

            hit.material.diffuse * light.intensity * light_facing_surface
        };

        return hit.material.color * (ambient_light + diffuse_light);
    }

    let unit_direction = ray.direction.normalize();
    let blend = 0.5 * (unit_direction.y + 1.0);

    let white = Vec3::new(1.0, 1.0, 1.0);
    let sky_blue = Vec3::new(0.5, 0.7, 1.0);

    white * (1.0 - blend) + sky_blue * blend
}

fn make_scene(scene_name: &str, aspect_ratio: f64) -> Option<(Scene, Camera, PointLight)> {
    let red = Material::new(Vec3::new(0.85, 0.15, 0.12), 0.12, 0.88);
    let green = Material::new(Vec3::new(0.12, 0.65, 0.20), 0.12, 0.88);
    let blue = Material::new(Vec3::new(0.12, 0.32, 0.85), 0.12, 0.88);
    let floor_gray = Material::new(Vec3::new(0.55, 0.55, 0.55), 0.12, 0.70);

    let mut scene = Scene::new();

    match scene_name {
        "sphere" => {
            scene.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5, red)));

            let camera = Camera::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 1.0, 0.0),
                aspect_ratio,
                45.0,
            );

            let light = PointLight::new(Vec3::new(-2.0, 2.5, 0.0), 1.0);

            Some((scene, camera, light))
        }

        "plane-cube" => {
            scene.add(Box::new(Cube::new(Vec3::new(0.0, -0.10, -1.5), 0.80, blue)));

            scene.add(Box::new(Plane::new(
                Vec3::new(0.0, -0.5, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                floor_gray,
            )));

            let camera = Camera::new(
                Vec3::new(2.1, 1.3, 2.2),
                Vec3::new(0.0, -0.1, -1.5),
                Vec3::new(0.0, 1.0, 0.0),
                aspect_ratio,
                45.0,
            );

            let light = PointLight::new(Vec3::new(-2.0, 2.5, 0.0), 0.45);

            Some((scene, camera, light))
        }

        "all" | "all-alt" => {
            scene.add(Box::new(Sphere::new(Vec3::new(0.25, 0.0, -1.25), 0.5, red)));

            scene.add(Box::new(Cylinder::new(
                Vec3::new(-0.85, -0.05, -1.45),
                0.30,
                0.90,
                green,
            )));

            scene.add(Box::new(Cube::new(
                Vec3::new(1.15, -0.10, -1.90),
                0.80,
                blue,
            )));

            scene.add(Box::new(Plane::new(
                Vec3::new(0.0, -0.5, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                floor_gray,
            )));

            let camera = if scene_name == "all" {
                Camera::new(
                    Vec3::new(2.8, 1.6, 2.8),
                    Vec3::new(0.0, -0.1, -1.4),
                    Vec3::new(0.0, 1.0, 0.0),
                    aspect_ratio,
                    45.0,
                )
            } else {
                Camera::new(
                    Vec3::new(-2.8, 1.8, 2.5),
                    Vec3::new(0.0, -0.1, -1.4),
                    Vec3::new(0.0, 1.0, 0.0),
                    aspect_ratio,
                    45.0,
                )
            };

            let light = PointLight::new(Vec3::new(-2.0, 2.5, 0.0), 1.0);

            Some((scene, camera, light))
        }

        _ => None,
    }
}

fn read_dimension(arguments: &[String], flag: &str, default_value: i32) -> i32 {
    let value = arguments
        .iter()
        .position(|argument| argument == flag)
        .and_then(|index| arguments.get(index + 1))
        .and_then(|value| value.parse::<i32>().ok());

    match value {
        Some(number) if number > 0 => number,
        _ => default_value,
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let scene_name = arguments.get(1).map(String::as_str).unwrap_or("all");

    let image_width = read_dimension(&arguments, "--width", 400);
    let image_height = read_dimension(&arguments, "--height", 225);
    let aspect_ratio = image_width as f64 / image_height as f64;

    let (scene, camera, light) = match make_scene(scene_name, aspect_ratio) {
        Some(result) => result,
        None => {
            eprintln!("Unknown scene: {}", scene_name);
            eprintln!("Available scenes: sphere, plane-cube, all, all-alt");
            return;
        }
    };

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let u = x as f64 / (image_width - 1) as f64;
            let v = y as f64 / (image_height - 1) as f64;

            let ray = camera.get_ray(u, v);
            write_color(ray_color(ray, &scene, &light));
        }
    }
}
