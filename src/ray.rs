use crate::vector3::{Vector3, Point3};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    //get origin/direction
    // pub fn origin(self) -> Vector3 {
    //     self.origin
    // }
    // pub fn direction(self) -> Vector3 {
    //     self.direction
    // }
    pub fn at(self, t:f64) -> Vector3 {
        self.origin + self.direction*t
    }
}