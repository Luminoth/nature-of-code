//! ECS resources

pub mod creatures;
pub mod debug;

use noise::{NoiseFn, Perlin, Seedable};
use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::prelude::*;

/// Random wrapper
pub struct Random {
    // TODO: would SmallRng be better here? we don't need a secur rng
    random: StdRng,
}

impl Random {
    /// Constructs a new random from a seed
    pub fn new(seed: u64) -> Self {
        Self {
            random: StdRng::seed_from_u64(seed),
        }
    }

    /// Generates a random value in the range 0..1
    pub fn random(&mut self) -> f64 {
        self.random_range(0.0..1.0)
    }

    /// Generates a random value in the given range
    pub fn random_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        self.random.gen_range(range)
    }
}

impl Default for Random {
    /// Constructs a default random from system entropy
    fn default() -> Self {
        Self {
            random: StdRng::from_entropy(),
        }
    }
}

/// Perlin noies wrapper
pub struct PerlinNoise {
    perlin: Perlin,
}

impl PerlinNoise {
    /// Constructs a new perlin noies function from the given seed
    #[allow(dead_code)]
    pub fn new(seed: u32) -> Self {
        Self {
            perlin: Perlin::new().set_seed(seed),
        }
    }

    /// Sample noise in the range [0..1],[0..1] scaled by frequency
    pub fn sample(&self, random: &mut Random, frequency: f64) -> f64 {
        self.perlin.get([random.random(), random.random()]) * frequency
    }
}

impl Default for PerlinNoise {
    /// Constructs a default perlin noise function from the thread local rng
    fn default() -> Self {
        Self {
            perlin: Perlin::new().set_seed(random()),
        }
    }
}
