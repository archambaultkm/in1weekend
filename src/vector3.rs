use std::ops;

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

    //get x, y, z
    pub fn x(self) -> f64 {
        self.x
    }

    pub fn y(self) -> f64 {
        self.y
    }

    pub fn z(self) -> f64 {
        self.z
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

    pub fn cross(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.y * rhs.z - self.z * rhs.y,
                     self.z * rhs.x - self.x * rhs.z,
                     self.x * rhs.y - self.y * rhs.x)
    }

    pub fn unit(self) -> Vector3 {
        self / self.length()
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, v: Vector3) -> Vector3 {
        Vector3::new(self.x + v.x, self.y + v.y, self.z + v.z)
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
