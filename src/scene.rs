use crate::objects::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: Ray, min_distance: f64, max_distance: f64) -> Option<HitRecord> {
        let mut closest_hit = None;
        let mut closest_distance = max_distance;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, min_distance, closest_distance) {
                closest_distance = hit.distance;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}
