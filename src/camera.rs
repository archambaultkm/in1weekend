use std::fs::File;
use std::io::Write;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector3::{Colour, Vector3};

pub struct Camera {
    pub origin: Vector3,
    image_height: f64,
    image_width: f64,
    pub pixel_delta_v: Vector3,
    pub pixel_delta_u: Vector3,
    pub pixel_origin: Vector3,
}

impl Camera {
    pub fn new(
        image_width:f64,
        image_height:f64,
        origin: Vector3,

    ) -> Camera {
        let focal_length = 1.0f64;
        let viewport_height = 2.0f64;
        let viewport_width = viewport_height * (image_width/image_height);

        let viewport_horizontal = Vector3 { x:viewport_width, y:0.0, z:0.0 };
        let viewport_vertical = Vector3 {x:0.00, y:viewport_height, z:0.0 };
        let pixel_delta_v = viewport_horizontal/image_width;
        let pixel_delta_u = viewport_vertical/image_height;

        let viewport_upper_left = origin
            - viewport_horizontal/2.0
            - viewport_vertical/2.0
            - Vector3::new(0.0, 0.0, focal_length);
        let pixel_origin = viewport_upper_left
            + viewport_horizontal * pixel_delta_u
            + viewport_vertical * pixel_delta_v;

        Camera {
            origin,
            image_height,
            image_width,
            pixel_delta_v,
            pixel_delta_u,
            pixel_origin,
        }
    }

    pub fn render(&self, world : &dyn Hittable) {
        let mut pixel_center : Vector3;
        let mut ray : Ray;
        let mut ray_direction : Vector3;

        //create a file
        let mut data_file = File::create("scene.ppm").expect("Creation failed.");

        //header required for ppm file
        data_file.write("P3\n".as_bytes()).expect("write failed");
        data_file.write((self.image_width.to_string() + " " + &*self.image_height.to_string() + "\n").as_bytes()).expect("write failed");
        data_file.write("255\n".as_bytes()).expect("write failed");

        //generate pixel colours in matrix and add to ppm file
        for i in (0..self.image_height as u32 +1).rev() {
            //progress tracker
            println!("On row {} of {}", self.image_height - i as f64, self.image_height);

            for j in 0..self.image_width as u32 {
                pixel_center = self.pixel_origin + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
                ray_direction = pixel_center - self.origin;
                ray = Ray{ origin : self.origin, direction:ray_direction };

                let pixel_colour = (ray_colour(&ray, world)).to_string();

                data_file.write((pixel_colour).as_bytes()).expect("write failed");
            }
        }

        println!("Created a file");
    }
}

fn ray_colour(ray:&Ray, world : &dyn Hittable) -> Colour {
    //calculate hit point and colour sphere according to its normal vectors
    let mut record = HitRecord::new();

    if world.hit(ray, Interval::new(0.0, f64::INFINITY), &mut record) {
        return (record.normal + Colour::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction : Vector3 = Vector3::unit(ray.direction);
    let a = 0.5 * (unit_direction.y + 1.0);
    //this is anything not hitting a shape: a white-to blue gradient background
    return Colour::new(1.0, 1.0, 1.0) * (1.0 - a) + Colour::new(0.5, 0.7, 1.0) * a;
}