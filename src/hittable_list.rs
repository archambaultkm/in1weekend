use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

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
    fn hit(&self, ray: &Ray, ray_t : Interval, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new();
        let mut hit_anything : bool = false;
        let mut closest_so_far = ray_t.max;


        for object in &self.objects {
            if object.hit(ray, ray_t, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                //TODO see if you can do like a copy constructor?
                record.t = temp_record.t;
                record.point = temp_record.point;
                record.front_face = temp_record.front_face;
                record.normal = temp_record.normal;
            }
        }

        return hit_anything;
    }
}