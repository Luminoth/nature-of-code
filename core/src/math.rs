use nalgebra::Vector2;
use num_traits::Float;
use rand::Rng;

use crate::*;

pub fn clamp<T: Ord>(v: T, min: T, max: T) -> T {
    std::cmp::min(max, std::cmp::max(min, v))
}

pub fn clampf<F: Float>(v: F, min: F, max: F) -> F {
    Float::min(max, Float::max(min, v))
}

// https://www.arduino.cc/reference/en/language/functions/math/map/
pub fn map<F: Float>(x: F, in_min: F, in_max: F, out_min: F, out_max: F) -> F {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn vector2_random() -> Vector2<f64> {
    let mut rng = rand::thread_rng();

    Vector2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize()
}

pub fn vector2_perlin() -> Vector2<f64> {
    Vector2::new(
        map(sample_noise2d(), 0.0, 1.0, -1.0, 1.0),
        map(sample_noise2d(), 0.0, 1.0, -1.0, 1.0),
    )
    .normalize()
}
