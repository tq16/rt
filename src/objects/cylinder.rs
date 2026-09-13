use crate::material::Material;
use crate::math::vec3::Vec3;
use crate::objects::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct Cylinder {
    pub center: Vec3,
    pub radius: f64,
    pub height: f64,
    pub material: Material,
}

impl Cylinder {
    pub fn new(center: Vec3, radius: f64, height: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            height,
            material,
        }
    }

    fn cap_hit(
        &self,
        ray: Ray,
        cap_y: f64,
        normal: Vec3,
        min_distance: f64,
        max_distance: f64,
    ) -> Option<HitRecord> {
        if ray.direction.y.abs() < 0.000001 {
            return None;
        }

        let distance = (cap_y - ray.origin.y) / ray.direction.y;

        if distance < min_distance || distance > max_distance {
            return None;
        }

        let point = ray.at(distance);
        let x_distance = point.x - self.center.x;
        let z_distance = point.z - self.center.z;

        if x_distance * x_distance + z_distance * z_distance > self.radius * self.radius {
            return None;
        }

        Some(HitRecord {
            point,
            normal,
            distance,
            material: self.material,
        })
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: Ray, min_distance: f64, max_distance: f64) -> Option<HitRecord> {
        let half_height = self.height / 2.0;
        let bottom_y = self.center.y - half_height;
        let top_y = self.center.y + half_height;

        let origin_to_center = ray.origin - self.center;

        let a = ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z;

        let half_b = origin_to_center.x * ray.direction.x + origin_to_center.z * ray.direction.z;

        let c = origin_to_center.x * origin_to_center.x + origin_to_center.z * origin_to_center.z
            - self.radius * self.radius;

        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_distance = max_distance;

        if a.abs() > 0.000001 {
            let discriminant = half_b * half_b - a * c;

            if discriminant >= 0.0 {
                let square_root = discriminant.sqrt();

                for distance in [(-half_b - square_root) / a, (-half_b + square_root) / a] {
                    if distance < min_distance || distance > closest_distance {
                        continue;
                    }

                    let point = ray.at(distance);

                    if point.y < bottom_y || point.y > top_y {
                        continue;
                    }

                    let normal = Vec3::new(point.x - self.center.x, 0.0, point.z - self.center.z)
                        .normalize();

                    closest_distance = distance;
                    closest_hit = Some(HitRecord {
                        point,
                        normal,
                        distance,
                        material: self.material,
                    });
                }
            }
        }

        if let Some(bottom_hit) = self.cap_hit(
            ray,
            bottom_y,
            Vec3::new(0.0, -1.0, 0.0),
            min_distance,
            closest_distance,
        ) {
            closest_distance = bottom_hit.distance;
            closest_hit = Some(bottom_hit);
        }

        if let Some(top_hit) = self.cap_hit(
            ray,
            top_y,
            Vec3::new(0.0, 1.0, 0.0),
            min_distance,
            closest_distance,
        ) {
            closest_hit = Some(top_hit);
        }

        closest_hit
    }
}
