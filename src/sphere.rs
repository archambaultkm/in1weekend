use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector3::*;

pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Sphere {
    pub fn new(center: Point3, radius : f64) -> Sphere {
        Sphere {
            center,
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t : Interval, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.squared_length();
        let half_b = oc.dot(ray.direction);
        let c = oc.squared_length() - self.radius * self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false;
        }

        let mut root = (-half_b - discriminant.sqrt()) / a;
        if ! ray_t.surrounds(root) {
            root = (-half_b + discriminant.sqrt()) / a;
            if ! ray_t.surrounds(root) {
                return false;
            }
        }

        //if the passed in ray hits this sphere, set the t, point and normal for shading
        record.t = root;
        record.point = ray.at(record.t);
        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);

        return true;
    }
}