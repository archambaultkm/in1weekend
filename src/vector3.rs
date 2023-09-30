use std::ops;

//base vector 3 struct used for colours and points.
//contains operation overrides
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,

}

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

impl ops::Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}