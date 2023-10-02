mod vector3;
mod ray;
mod camera;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;

use crate::camera::Camera;
use crate::hittable::{Hittable};
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vector3::{Point3, Vector3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const MAX_COLOUR: u8 = 255;


fn main() {
    //create camera
    let camera : Camera = Camera::new(
        IMAGE_WIDTH as f64,
        IMAGE_HEIGHT as f64,
        Vector3 { x: 0.0, y: 0.0, z: 0.0 });

    //World
    let mut world = HittableList::new();

    //TODO I have to add objects in the opposite order to the book- figure out what's causing that
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5
    )));

    camera.render(&world);
}