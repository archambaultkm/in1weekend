use std::sync::Arc;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::material::*;
use crate::vector3::*;

#[derive(Clone)]
pub struct HitRecord {
    pub point : Point3,
    pub normal : Vector3,
    pub material : Arc<dyn Material>,
    pub t : f64,
    pub front_face : bool
}

impl HitRecord {
    pub fn new(point: Point3, normal: Vector3, t: f64, material: Arc<dyn Material>) -> Self {
        HitRecord {
            point,
            normal,
            t,
            front_face: false,
            material
        }
    }

    pub fn set_face_normal(&mut self, ray : &Ray, outward_normal : Vector3) {
        //sets the hit record normal vector
        //note the parameter 'outward normal' is assumed to have unit length

        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray : &Ray, ray_t : Interval) -> Option<HitRecord>;
}