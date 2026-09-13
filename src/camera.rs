use crate::math::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        world_up: Vec3,
        aspect_ratio: f64,
        field_of_view: f64,
    ) -> Self {
        let field_of_view_radians = field_of_view.to_radians();
        let viewport_height = 2.0 * (field_of_view_radians / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let forward = (look_at - look_from).normalize();
        let right = forward.cross(world_up).normalize();
        let up = right.cross(forward).normalize();

        let focal_length = 1.0;
        let viewport_center = look_from + forward * focal_length;

        let horizontal = right * viewport_width;
        let vertical = up * viewport_height;

        let lower_left_corner =
            viewport_center
            - horizontal / 2.0
            - vertical / 2.0;

        Self {
            origin: look_from,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction =
            self.lower_left_corner
            + self.horizontal * u
            + self.vertical * v
            - self.origin;

        Ray::new(self.origin, direction)
    }
}