use rand::{Rng, thread_rng};

pub fn random() -> f64 {
    thread_rng().gen()
}