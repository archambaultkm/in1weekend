use std::fs::File;
use std::io::Write;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::util;
use crate::vector3::{Colour, Point3, Vector3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
pub const MAX_COLOUR: f64 = 255.99;
const SAMPLES_PER_PIXEL : i32 = 100;
const MAX_DEPTH : i32 = 50;
const VFOV : f64 = 50.0; //vertical view angle (field of view)
const VUP : Vector3 = Vector3{x: 0.0, y: 1.0, z: 0.0}; // Camera-relative "up" direction

pub struct Camera {
    pub origin: Vector3,
    image_height: f64,
    image_width: f64,
    pub pixel_delta_v: Vector3,
    pub pixel_delta_u: Vector3,
    pub pixel_origin: Vector3
}

impl Camera {
    pub fn new(
        look_from : Point3,
        look_at : Point3

    ) -> Camera {
        // Initialize
        let origin = look_from;

        //Determine viewport dimensions
        let focal_length = (look_from - look_at).length();
        let theta = VFOV.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * ASPECT_RATIO;

        //Calculate u,v,w unit basis vectors for the camera coordinate frame
        let w = (look_from - look_at).unit();
        let u = VUP.cross(w).unit();
        let v = w.cross(u);

        //calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_horizontal = u * viewport_width * focal_length;
        let viewport_vertical = v * viewport_height * focal_length;

        //calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_v = viewport_horizontal/IMAGE_WIDTH as f64;
        let pixel_delta_u = viewport_vertical/IMAGE_HEIGHT as f64;

        //calculate the position of the upper left pixel
        let viewport_upper_left = origin
            - viewport_horizontal/2.0
            - viewport_vertical/2.0
            - (w * focal_length);
        let pixel_origin = viewport_upper_left
            + viewport_horizontal * pixel_delta_u
            + viewport_vertical * pixel_delta_v;

        Camera {
            origin,
            image_height : IMAGE_HEIGHT as f64,
            image_width : IMAGE_WIDTH as f64,
            pixel_delta_v,
            pixel_delta_u,
            pixel_origin
        }
    }

    pub fn render(&self, world : &dyn Hittable) {
        let mut ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));

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
                let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);

                for sample in 0..SAMPLES_PER_PIXEL {
                    ray = self.get_ray(i, j);
                    pixel_colour += ray_colour(&ray, MAX_DEPTH, world);
                }

                let pixel_colour = pixel_colour.to_string(SAMPLES_PER_PIXEL);

                data_file.write((pixel_colour).as_bytes()).expect("write failed");
            }
        }

        println!("Created a file");
    }

    // Returns a random point in the square surrounding a pixel at the origin
    fn pixel_sample_square(&self) -> Vector3 {
        let px = util::random() - 0.5;
        let py = util::random() - 0.5;

        return (self.pixel_delta_u * px) + (self.pixel_delta_v * py);
    }

    //Get a randomly sampled camera ray for the pixel at location i,j
    fn get_ray(&self, i: u32, j : u32) -> Ray {
        let pixel_center = self.pixel_origin + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.origin;
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(self.origin, ray_direction);
    }

}

fn ray_colour(ray:&Ray, depth : i32,  world : &dyn Hittable) -> Colour {

    //ensure function doesn't recurse forever (stop gathering light if at max depth)
    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    //calculate hit point and colour sphere according to its normal vectors
    //ignore hits very close to the calculated intersection point (range starts at 0.001) for the shadow acne
    if let Some(record) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {

        return if let Some((attenuation, scattered)) = record.material.scatter(ray, &record) {
            attenuation * ray_colour(&scattered, depth - 1, world)
        } else {
            Colour::new(0.0, 0.0, 0.0)
        };
    }

    let unit_direction : Vector3 = Vector3::unit(ray.direction);
    let a = 0.5 * (unit_direction.y + 1.0);

    //this is anything not hitting a shape: a white-to blue gradient background
    return Colour::new(1.0, 1.0, 1.0) * (1.0 - a) + Colour::new(0.5, 0.7, 1.0) * a;
}