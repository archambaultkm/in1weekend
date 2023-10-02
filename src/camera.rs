use crate::ray::Ray;
use crate::vector3::Vector3;

pub struct Camera {
    pub origin:Vector3,
    focal_length:f64,
    viewport_height:f64,
    viewport_width:f64,
    viewport_horizontal:Vector3,
    viewport_vertical:Vector3,
    pub pixel_delta_v:Vector3,
    pub pixel_delta_u:Vector3,
    pub pixel_origin:Vector3,
}

//these equations are from Ray Tracing in One Weekend: Listing 9
impl Camera {
    pub fn new(
        image_width:f64,
        image_height:f64,
        origin:Vector3,

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
            - Vector3::new(0.0,0.0,focal_length);
        let pixel_origin = viewport_upper_left
            + viewport_horizontal * pixel_delta_u
            + viewport_vertical * pixel_delta_v;

        Camera {
            origin,
            focal_length,
            viewport_height,
            viewport_width,
            viewport_horizontal,
            viewport_vertical,
            pixel_delta_v,
            pixel_delta_u,
            pixel_origin,
        }
    }
}