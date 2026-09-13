use crate::math::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Vec3,
    pub ambient: f64,
    pub diffuse: f64,
}

impl Material {
    pub fn new(color: Vec3, ambient: f64, diffuse: f64) -> Self {
        Self {
            color,
            ambient,
            diffuse,
        }
    }
}
