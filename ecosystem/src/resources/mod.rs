//! ECS resources

pub mod creatures;

use noise::{NoiseFn, Perlin, Seedable};
use rand::random;

pub struct Noise {
    perlin: Perlin,
}

impl Noise {
    pub fn get_perlin(&self, point: f64) -> f64 {
        self.perlin.get([point, 0.0])
    }
}

impl Default for Noise {
    fn default() -> Self {
        Self {
            perlin: Perlin::new().set_seed(random()),
        }
    }
}
