use crate::material::Material;
use crate::math::vec3::Vec3;
use crate::objects::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct Cube {
    pub center: Vec3,
    pub size: f64,
    pub material: Material,
}

impl Cube {
    pub fn new(center: Vec3, size: f64, material: Material) -> Self {
        Self {
            center,
            size,
            material,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: Ray, min_distance: f64, max_distance: f64) -> Option<HitRecord> {
        let half_size = self.size / 2.0;

        let min = self.center - Vec3::new(half_size, half_size, half_size);
        let max = self.center + Vec3::new(half_size, half_size, half_size);

        let mut enter_distance = min_distance;
        let mut exit_distance = max_distance;

        let mut enter_normal = Vec3::zero();
        let mut exit_normal = Vec3::zero();

        let axes = [
            (
                ray.origin.x,
                ray.direction.x,
                min.x,
                max.x,
                Vec3::new(-1.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            (
                ray.origin.y,
                ray.direction.y,
                min.y,
                max.y,
                Vec3::new(0.0, -1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            (
                ray.origin.z,
                ray.direction.z,
                min.z,
                max.z,
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
        ];

        for (origin, direction, minimum, maximum, min_normal, max_normal) in axes {
            if direction.abs() < 0.000001 {
                if origin < minimum || origin > maximum {
                    return None;
                }

                continue;
            }

            let first_distance = (minimum - origin) / direction;
            let second_distance = (maximum - origin) / direction;

            let (near_distance, far_distance, near_normal, far_normal) =
                if first_distance < second_distance {
                    (first_distance, second_distance, min_normal, max_normal)
                } else {
                    (second_distance, first_distance, max_normal, min_normal)
                };

            if near_distance > enter_distance {
                enter_distance = near_distance;
                enter_normal = near_normal;
            }

            if far_distance < exit_distance {
                exit_distance = far_distance;
                exit_normal = far_normal;
            }

            if enter_distance > exit_distance {
                return None;
            }
        }

        let (distance, normal) = if enter_distance >= min_distance {
            (enter_distance, enter_normal)
        } else {
            (exit_distance, exit_normal)
        };

        if distance < min_distance || distance > max_distance {
            return None;
        }

        Some(HitRecord {
            point: ray.at(distance),
            normal,
            distance,
            material: self.material,
        })
    }
}