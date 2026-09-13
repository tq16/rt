use crate::material::Material;
use crate::math::vec3::Vec3;
use crate::objects::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Material) -> Self {
        Self {
            point,
            normal: normal.normalize(),
            material,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: Ray, min_distance: f64, max_distance: f64) -> Option<HitRecord> {
        let ray_facing_plane = self.normal.dot(ray.direction);

        if ray_facing_plane.abs() < 0.000001 {
            return None;
        }

        let distance = (self.point - ray.origin).dot(self.normal) / ray_facing_plane;

        if distance < min_distance || distance > max_distance {
            return None;
        }

        Some(HitRecord {
            point: ray.at(distance),
            normal: self.normal,
            distance,
            material: self.material,
        })
    }
}
