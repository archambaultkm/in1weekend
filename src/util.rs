use crate::interval::Interval;

pub fn random() -> f64 {
    rand::random()
}

pub fn random_in_interval(range : Interval) -> f64 {
    range.min + (range.max - range.min) * random()
}