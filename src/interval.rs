#[derive(Clone, Copy)]
pub struct Interval {
    pub min : f64,
    pub max : f64
}

impl Interval {
    pub fn new(min : f64, max: f64) -> Interval {
        Interval {
            min,
            max
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        return self.min <= x && self.max >= x;
    }

    pub fn surrounds(&self, x : f64) -> bool {
        return self.min < x && x < self.max;
    }
}