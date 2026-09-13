use crate::material::Material;
use crate::math::vec3::Vec3;
use crate::objects::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, min_distance: f64, max_distance: f64) -> Option<HitRecord> {
        let origin_to_center = ray.origin - self.center;

        let a = ray.direction.dot(ray.direction);
        let half_b = origin_to_center.dot(ray.direction);
        let c = origin_to_center.dot(origin_to_center) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let square_root = discriminant.sqrt();

        let mut distance = (-half_b - square_root) / a;

        if distance < min_distance || distance > max_distance {
            distance = (-half_b + square_root) / a;

            if distance < min_distance || distance > max_distance {
                return None;
            }
        }

        let point = ray.at(distance);
        let normal = (point - self.center) / self.radius;

        Some(HitRecord {
            point,
            normal,
            distance,
            material: self.material,
        })
    }
}