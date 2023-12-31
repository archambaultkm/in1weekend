use std::ops;
use crate::camera::MAX_COLOUR;
use crate::interval::Interval;
use crate::util;

#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vector3;
pub type Colour = Vector3;
// TODO see if there's a way to change the field names for different types for rgb

impl Vector3 {
    //simple constructor
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn squared_length(self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn length(self) -> f64 {
        (self.squared_length()).sqrt()
    }

    pub fn dot(self, v: Vector3) -> f64 {
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
    }

    pub fn cross(self, v: Vector3) -> Vector3 {
        Vector3::new(self.y * v.z - self.z * v.y,
                     self.z * v.x - self.x * v.z,
                     self.x * v.y - self.y * v.x)
    }

    pub fn unit(self) -> Vector3 {
        self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (f64::abs(self.x) < s) && (f64::abs(self.y) < s) && (f64::abs(self.z) < s)
    }

    pub fn reflect(&self, n : &Vector3) -> Vector3 {
        return *self - *n * Vector3::dot(*self, *n)*2.0;
    }

    // TODO write out explanation for this
    pub fn refract(&self, n : &Vector3, etai_over_etat : f64) -> Vector3 {
        let cos_theta = f64::min(-self.dot(*n), 1.0);

        let r_out_perpendicular = (*n * cos_theta + *self) * etai_over_etat;
        let r_out_parallel = *n * -f64::abs(1.0 - r_out_perpendicular.squared_length()).sqrt();

        return r_out_perpendicular + r_out_parallel;
    }
}

impl Colour {
    pub fn to_string(&self, samples_per_pixel : i32) -> String {
        let scale = 1.0 / samples_per_pixel as f64;
        let r = linear_to_gamma(self.x * scale);
        let g = linear_to_gamma(self.y * scale);
        let b = linear_to_gamma(self.z * scale);

        let intensity = Interval::new(0.000, 0.999);
        return ((intensity.clamp(r) * MAX_COLOUR) as i32).to_string() + " " +
            &*((intensity.clamp(g) * MAX_COLOUR) as i32).to_string() + " " +
            &*((intensity.clamp(b) * MAX_COLOUR) as i32).to_string() + "\n"
    }
}

// images rendered are darker than expected because they are expected to be in "gamma space",
// meaning it assumes the image has been transformed. transform linear to gamma so that the renderer
// displays the expected image https://raytracing.github.io/books/RayTracingInOneWeekend.html#diffusematerials/usinggammacorrectionforaccuratecolorintensity
pub fn linear_to_gamma(linear_component : f64) -> f64 {
    return linear_component.sqrt();
}

pub fn random() -> Vector3 {
    return Vector3::new(
        util::random(),
        util::random(),
        util::random())
}

pub fn random_in_interval(range : Interval) -> Vector3 {
    return Vector3::new(
        util::random_in_interval(range),
        util::random_in_interval(range),
        util::random_in_interval(range)
    )
}

pub fn random_in_unit_sphere() -> Vector3 {
    loop {
        let p = random_in_interval(Interval::new(-1.0, 1.0));

        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vector3 {
    return Vector3::unit(random_in_unit_sphere());
}

pub fn random_on_hemisphere(normal : Vector3) -> Vector3 {
    let on_unit_sphere = random_unit_vector();

    return if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn random_in_unit_disk() -> Vector3 {
    loop {
        let p = Vector3::new(
            util::random_in_interval(Interval::new(-1.0, 1.0)),
            util::random_in_interval(Interval::new(-1.0, 1.0)),
            0.0
        );

        if p.squared_length() >= 1.0 {
            continue;
        }
        return p;
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, v: Vector3) -> Vector3 {
        Vector3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl ops::AddAssign<Vector3> for Vector3 {

    fn add_assign(&mut self, v: Vector3) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, v: Vector3) -> Vector3 {
        Vector3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Vector3 {
        Vector3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

impl ops::Div<Vector3> for Vector3 {
    type Output = Vector3;
    fn div(self, v: Vector3) -> Vector3 {
        Vector3::new(self.x / v.x, self.y / v.y, self.z / v.z)
    }
}

//matrix division and multiplication by a constant is just each index */ the float
impl ops::Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, f: f64) -> Vector3 { Vector3::new(self.x / f, self.y / f, self.z / f)}
}

impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, f: f64) -> Vector3 { Vector3::new(self.x * f, self.y * f, self.z * f)}
}

impl ops::Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}
