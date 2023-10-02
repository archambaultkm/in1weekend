mod vector3;
mod ray;
mod camera;

use std::fs::File;
use std::io::Write;
use crate::camera::Camera;
use crate::ray::Ray;
use crate::vector3::Point3;
use crate::vector3::Colour;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const MAX_COLOUR: u8 = 255;

//TODO move out of here to colours or materials or something
fn string_colour(pixel_colour: Colour) -> String {
    return ((pixel_colour.r * 255.99) as i32).to_string() + " " +
        &*((pixel_colour.g * 255.99) as i32).to_string() + " " +
        &*((pixel_colour.b * 255.99) as i32).to_string() + "\n"
}

fn hit_sphere(center : &Point3, radius : f64, ray : &Ray) -> f64 {
    let oc = ray.origin - *center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius  * radius;

    let discriminant = b*b - 4.0*a*c;

    //if it doesn't hit, discriminant is less than zero
    return if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_colour(ray:&Ray) -> Colour {
    //calculate hit point and colour sphere according to its normal vectors
    let t = hit_sphere(&Point3 { x: 0.0, y: 0.0, z: -1.0}, 0.5, ray);

    //shade based on normals
    if t > 0.0 {
        let n = Point3::unit(ray.at(t) - Point3 { x: 0.0, y: 0.0, z: -1.0});
        return Colour::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
    }

    let unit_direction : Point3 = Point3::unit(ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    //this is anything not hitting a shape: a white-to blue gradient background
    return Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.5, 0.7, 1.0) * t
}

fn main() {
    //create camera
    let camera : Camera = Camera::new(
        IMAGE_WIDTH as f64,
        IMAGE_HEIGHT as f64,
        Point3 { x: 0.0, y: 0.0, z: 0.0 });

    //create canvas
    let cols = IMAGE_WIDTH;
    let rows = IMAGE_HEIGHT;
    let mut pixel_center : Point3;
    let mut ray : Ray;
    let mut ray_direction : Point3;

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

            let pixel_colour = string_colour(ray_colour(&ray));
            data_file.write((pixel_colour).as_bytes()).expect("write failed");
        }
    }

    println!("Created a file");
}