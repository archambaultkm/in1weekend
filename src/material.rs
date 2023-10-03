use std::ops::DivAssign;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::util;
use crate::vector3::{Colour, random_in_unit_sphere, random_unit_vector};

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

pub struct Dielectric {
    ir : f64 //Index of Refraction
}

impl Dielectric {
    pub fn new(ir : f64) -> Dielectric {
        Dielectric {
            ir
        }
    }

    pub fn reflectance(cosine : f64, refraction_idx : f64) -> f64 {
        //Schlick's approximation for reflectance
        let mut r0 = (1.0 - refraction_idx) / (1.0 + refraction_idx);
        r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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
        let reflected = ray.direction.unit().reflect(&record.normal);
        let scattered = Ray::new(record.point, reflected + random_in_unit_sphere() * self.fuzz);

        if scattered.direction.dot(record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Colour, Ray)> {
        let attenuation = Colour::new(1.0, 1.0, 1.0);
        let refraction_ratio = if record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray.direction.unit();
        let cos_theta = f64::min((-unit_direction).dot(record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract || (Dielectric::reflectance(cos_theta, refraction_ratio) > util::random()) {
            unit_direction.reflect(&record.normal)
        } else {
            unit_direction.refract(&record.normal, refraction_ratio)
        };

        let scattered = Ray::new(record.point, direction);
        Some((attenuation, scattered))
    }
}