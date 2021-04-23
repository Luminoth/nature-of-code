use nalgebra::Vector2;
use rand::Rng;

pub fn clamp<T: Ord>(v: T, min: T, max: T) -> T {
    std::cmp::min(max, std::cmp::max(min, v))
}

// https://www.arduino.cc/reference/en/language/functions/math/map/
pub fn map(x: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn vector2_random() -> Vector2<f64> {
    let mut rng = rand::thread_rng();

    Vector2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize()
}
