use std::f64::consts::PI;

#[inline]
pub fn degrees_to_radians(deg: f64) -> f64 {
    debug_assert!(deg > 0.0 && deg <= 360.0);
    deg * PI / 180.0
}

#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    match x {
        x if x < min => min,
        x if x > max => max,
        _ => x,
    }
}

// Reexporting from random (because reasons)
pub use rand::random;
use rand::{
    distributions::{
        uniform::{SampleRange, SampleUniform},
        Standard,
    },
    prelude::Distribution,
    Rng,
};

pub fn rand_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}
