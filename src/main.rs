mod vector3;
mod ray;
mod camera;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod util;
mod material;

use std::f32::consts::PI;
use std::sync::Arc;
use crate::camera::Camera;
use crate::hittable::{Hittable};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Matte, Metal};
use crate::sphere::Sphere;
use crate::vector3::{Colour, Point3, Vector3};

fn main() {
    //World
    let mut world = HittableList::new();

    //let r = f64::cos((PI / 4.0) as f64);

    let material_ground = Arc::new(Matte::new(Colour::new(
        0.8,
        0.8,
        0.0
    )));

    let material_matte = Arc::new(Matte::new(Colour::new(
        0.1,
        0.2,
        0.5
    )));

    let material_glass = Arc::new(Dielectric::new(1.5));

    let material_metal = Arc::new(Metal::new(Colour::new(
        0.8,
        0.6,
        0.2
    ), 0.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone()
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(1.2, 0.0, -1.0),
        0.5,
        material_metal.clone()
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_matte.clone()
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.5,
        material_glass.clone()
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        material_glass.clone()
    )));

    //create camera + render scene
    let camera : Camera = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0)
    );

    camera.render(&world);
}