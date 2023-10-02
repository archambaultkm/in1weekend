use std::ops;

enum Vector3 {
    Point3(f64, f64, f64),
    Colour(f64, f64, f64)
}

#[derive(Clone, Copy, Debug)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 {
    //simple constructor
    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3 { x, y, z }
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

    pub fn dot(self, v: Point3) -> f64 {
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
    }

    pub fn cross(self, rhs: Point3) -> Point3 {
        Point3::new(self.y * rhs.z - self.z * rhs.y,
                    self.z * rhs.x - self.x * rhs.z,
                    self.x * rhs.y - self.y * rhs.x)
    }

    pub fn unit(self) -> Point3 {
        self / self.length()
    }
}

impl ops::Add<Point3> for Point3 {
    type Output = Point3;
    fn add(self, v: Point3) -> Point3 {
        Point3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl ops::Sub<Point3> for Point3 {
    type Output = Point3;
    fn sub(self, v: Point3) -> Point3 {
        Point3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl ops::Mul<Point3> for Point3 {
    type Output = Point3;
    fn mul(self, v: Point3) -> Point3 {
        Point3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

impl ops::Div<Point3> for Point3 {
    type Output = Point3;
    fn div(self, v: Point3) -> Point3 {
        Point3::new(self.x / v.x, self.y / v.y, self.z / v.z)
    }
}

//matrix division and multiplication by a constant is just each index */ the float
impl ops::Div<f64> for Point3 {
    type Output = Point3;
    fn div(self, f: f64) -> Point3 { Point3::new(self.x / f, self.y / f, self.z / f)}
}

impl ops::Mul<f64> for Point3 {
    type Output = Point3;
    fn mul(self, f: f64) -> Point3 { Point3::new(self.x * f, self.y * f, self.z * f)}
}

impl ops::Neg for Point3 {
    type Output = Point3;
    fn neg(self) -> Point3 {
        Point3::new(-self.x, -self.y, -self.z)
    }
}


pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    //simple constructor
    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour { r, g, b }
    }
}

impl ops::Mul<f64> for Colour {
    type Output = Colour;
    fn mul(self, f: f64) -> Colour { Colour::new(self.r * f, self.g * f, self.b * f)}
}

impl ops::Add<Colour> for Colour {
    type Output = Colour;
    fn add(self, v: Colour) -> Colour {
        Colour::new(self.r + v.r, self.g + v.g, self.b + v.b)
    }
}