use crate::math::vec3::Vec3;

pub struct PointLight {
    pub position: Vec3,
    pub intensity: f64,
}

impl PointLight {
    pub fn new(position: Vec3, intensity: f64) -> Self {
        Self {
            position,
            intensity,
        }
    }
}
