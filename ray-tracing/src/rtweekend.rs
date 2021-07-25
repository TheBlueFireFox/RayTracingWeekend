use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    Rng,
};
use std::f64::consts::PI;

#[inline]
pub fn degrees_to_radians(deg: f64) -> f64 {
    debug_assert!(deg > 0.0 && deg <= 360.0);
    deg * PI / 180.0
}

#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    match x {
        i if i < min => min,
        i if i > max => max,
        _ => x,
    }
}

pub fn rand_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    rand::thread_rng().gen_range(range)
}
