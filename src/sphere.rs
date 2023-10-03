use std::sync::Arc;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector3::*;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material : Arc<dyn Material>
}

impl Sphere {
    pub fn new(center: Point3, radius : f64, material : Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t : Interval) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.squared_length();
        let half_b = oc.dot(ray.direction);
        let c = oc.squared_length() - self.radius * self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return None;
        }

        let mut root = (-half_b - discriminant.sqrt()) / a;
        if ! ray_t.surrounds(root) {
            root = (-half_b + discriminant.sqrt()) / a;
            if ! ray_t.surrounds(root) {
                return None;
            }
        }

        //if the passed in ray hits this sphere, set the t, point and normal for shading
        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;

        let mut record = HitRecord::new(
            point,
            outward_normal,
            t,
            Arc::clone(&self.material)
        );

        record.set_face_normal(ray, outward_normal);

        return Some(record);
    }
}