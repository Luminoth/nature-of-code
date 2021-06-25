use glam::DVec2;
use num_traits::Float;
use rand::Rng;

//use crate::*;

pub fn clamp<T: Ord>(v: T, min: T, max: T) -> T {
    std::cmp::min(max, std::cmp::max(min, v))
}

pub fn clampf<F: Float>(v: F, min: F, max: F) -> F {
    Float::min(max, Float::max(min, v))
}

// https://www.arduino.cc/reference/en/language/functions/math/map/
pub fn map<F: Float>(v: F, in_min: F, in_max: F, out_min: F, out_max: F) -> F {
    (v - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn vector2_random() -> DVec2 {
    let mut rng = rand::thread_rng();

    DVec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize_or_zero()
}

pub fn vector2_random_angle() -> DVec2 {
    let mut rng = rand::thread_rng();

    let theta = rng.gen_range(0.0..std::f64::consts::PI * 2.0);
    DVec2::new(theta.cos(), theta.sin()).normalize_or_zero()
}

/*pub fn vector2_perlin(frequency: f64) -> DVec2 {
    // TODO: should we take 2 frequencies here (one for x, one for y) ?
    DVec2::new(sample_noise2d(frequency), sample_noise2d(frequency)).normalize_or_zero()
}*/

/// Project ap onto ab
pub fn project(p: DVec2, a: DVec2, b: DVec2) -> DVec2 {
    let ap = p - a;
    let ab = (b - a).normalize_or_zero();

    let proj = ab * ap.dot(ab);
    a + proj
}
