mod vector3;
mod ray;
mod camera;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod util;
mod material;

use std::sync::Arc;
use crate::camera::Camera;
use crate::hittable::{Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::{Dielectric, Matte, Metal};
use crate::sphere::Sphere;
use crate::vector3::{Colour, Point3, Vector3};

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Matte::new(Colour::new(0.5, 0.5, 0.5));
    let sphere = Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(ground_material),
    );
    world.add(Box::new(sphere));

    let some_point = Point3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = util::random();
            let center = Point3::new(a as f64 + 0.9 * util::random(), 0.2, b as f64 + 0.9 * util::random());

            if (center - some_point).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = vector3::random() * vector3::random();
                    let sphere_material = Matte::new(albedo);
                    let sphere = Sphere::new(center, 0.2, Arc::new(sphere_material));
                    world.add(Box::new(sphere));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = vector3::random_in_interval(Interval::new(0.0, 0.5));
                    let fuzz = util::random_in_interval(Interval::new(0.0, 0.5));
                    let sphere_material = Metal::new(albedo, fuzz);
                    let sphere = Sphere::new(center, 0.2, Arc::new(sphere_material));
                    world.add(Box::new(sphere));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    let sphere = Sphere::new(center, 0.2, Arc::new(sphere_material));
                    world.add(Box::new(sphere));
                }
            }
        }
    }

    let sphere_material = Matte::new(Colour::new(0.9, 0.2, 0.3));
    let sphere = Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, Arc::new(sphere_material));
    world.add(Box::new(sphere));

    let sphere_material = Dielectric::new(1.5);
    let sphere = Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, Arc::new(sphere_material));
    world.add(Box::new(sphere));

    let sphere_material = Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0);
    let sphere = Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, Arc::new(sphere_material));
    world.add(Box::new(sphere));

    world
}

fn main() {
    //World
    let world = random_scene();

    //create camera + render scene
    let camera : Camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0), // look from
        Point3::new(0.0, 0.0, 0.0) // look at
    );

    camera.render(&world);
}