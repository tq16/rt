use crate::material::Material;
use crate::math::vec3::Vec3;
use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f64,
    pub material: Material,
}

pub trait Hittable {
    fn hit(&self, ray: Ray, min_distance: f64, max_distance: f64) -> Option<HitRecord>;
}