use std::sync::Arc;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Matte;
use crate::ray::Ray;
use crate::vector3::{Colour, Vector3};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: vec![],
        }
    }

    pub fn add(&mut self, object : Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t : Interval) -> Option<HitRecord> {
        let mut record: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(object_record) = object.hit(ray, ray_t) {
                closest_so_far = object_record.t;
                record = Some(object_record);
            }
        }

        return record;
    }
}