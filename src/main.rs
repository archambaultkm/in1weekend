mod vector3;
mod ray;
mod camera;

use std::fs::File;
use std::io::Write;
use crate::camera::Camera;
use crate::ray::Ray;
use crate::vector3::Vector3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const MAX_COLOUR: u8 = 255;

//TODO move out of here to colours or materials or something
fn string_colour(pixel_colour:Vector3) -> String {
    return ((pixel_colour.x * 255.99) as i32).to_string() + " " +
        &*((pixel_colour.y * 255.99) as i32).to_string() + " " +
        &*((pixel_colour.z * 255.99) as i32).to_string() + "\n"
}

fn ray_colour(ray:Ray) -> Vector3 {
    let unit_direction : Vector3 = Vector3::unit(ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    //TODO figure out what this means
    return Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    //create camera
    let camera : Camera = Camera::new(
        IMAGE_WIDTH as f64,
        IMAGE_HEIGHT as f64,
        Vector3{ x: 0.0, y: 0.0, z: 0.0 });

    //create canvas
    let cols = IMAGE_WIDTH;
    let rows = IMAGE_HEIGHT;
    let mut pixel_center : Vector3;
    let mut ray : Ray;
    let mut ray_direction : Vector3;

    //create a file
    let mut data_file = File::create("scene.ppm").expect("Creation failed.");

    //header required for ppm file
    data_file.write("P3\n".as_bytes()).expect("write failed");
    data_file.write((IMAGE_WIDTH.to_string() + " " + &*IMAGE_HEIGHT.to_string() + "\n").as_bytes()).expect("write failed");
    data_file.write("255\n".as_bytes()).expect("write failed");

    //generate pixel colours in matrix and add to ppm file
    for i in (0..rows+1).rev() {
        for j in 0..cols {
            pixel_center = camera.pixel_origin + (camera.pixel_delta_u * i as f64) + (camera.pixel_delta_v * j as f64);
            ray_direction = pixel_center - camera.origin;
            ray = Ray{ origin:camera.origin, direction:ray_direction };

            let pixel_colour = string_colour(ray_colour(ray));
            data_file.write((pixel_colour).as_bytes()).expect("write failed");
        }
    }

    println!("Created a file");
}