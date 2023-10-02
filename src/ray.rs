use crate::vector3::Point3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Point3
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Ray {
        Ray { origin, direction }
    }

    //get origin/direction
    pub fn origin(self) -> Point3 {
        self.origin
    }
    pub fn direction(self) -> Point3 {
        self.direction
    }

    pub fn at(self, t:f64) -> Point3 {
        self.origin + self.direction*t
    }
}