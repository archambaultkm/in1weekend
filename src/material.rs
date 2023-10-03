use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector3::{Colour, random_in_unit_sphere, random_unit_vector, reflect, Vector3};

pub trait Material {
    fn scatter(&self, ray : &Ray, record:  &HitRecord) -> Option<(Colour, Ray)>;
}

pub struct Matte {
    albedo : Colour
}

impl Matte {
    pub fn new(albedo : Colour) -> Matte {
        Matte {
            albedo
        }
    }
}

pub struct Metal {
    albedo : Colour,
    fuzz : f64
}

impl Metal {
    pub fn new(albedo : Colour, fuzz : f64) -> Metal {
        Metal {
            albedo,
            fuzz
        }
    }
}

impl Material for Matte {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Colour, Ray)> {
        let mut scatter_direction = record.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        let scattered = Ray::new(record.point, scatter_direction);

        Some((self.albedo, scattered))
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Colour, Ray)> {
        let reflected = reflect(ray.direction.unit(), record.normal);
        let scattered = Ray::new(record.point, reflected + random_in_unit_sphere() * self.fuzz);

        if scattered.direction().dot(record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}