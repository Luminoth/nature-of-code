//! ECS resources

pub mod debug;

use bevy::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use num_traits::Float;
use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::prelude::*;
use rand_distr::{Normal, StandardNormal};

use crate::clampf;

/// World boundary
pub struct WorldBounds {
    pub width: f32,
    pub height: f32,
}

/// Simulation parameters
pub struct SimulationParams {
    pub fly_count: usize,
    pub fish_count: usize,
    pub snake_count: usize,
}

/// Random wrapper
pub struct Random {
    // TODO: would SmallRng be better here? we don't need a secure rng
    random: StdRng,
}

impl Default for Random {
    /// Constructs a default random from system entropy
    fn default() -> Self {
        Self {
            random: StdRng::from_entropy(),
        }
    }
}

impl Random {
    /// Constructs a new random from a seed
    #[allow(dead_code)]
    pub fn new(seed: u64) -> Self {
        Self {
            random: StdRng::seed_from_u64(seed),
        }
    }

    /// Coin returns a random boolean
    #[allow(dead_code)]
    pub fn coin(&mut self) -> bool {
        self.random_range(0..=1) == 1
    }

    /// Dice returns a random in the range [1..faces]
    #[allow(dead_code)]
    pub fn dice(&mut self, faces: usize) -> usize {
        self.random_range(1..=faces)
    }

    /// Generates a uniform random value in the range [0..1)
    #[allow(dead_code)]
    pub fn random(&mut self) -> f64 {
        self.random_range(0.0..1.0)
    }

    /// Generates a uniform random value in the given range
    #[allow(dead_code)]
    pub fn random_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        self.random.gen_range(range)
    }

    /// Generates a uniform random vector in the range ([0..1], [0..1])
    #[allow(dead_code)]
    pub fn vec2(&mut self) -> Vec2 {
        self.vec2_range(0.0..=1.0, 0.0..=1.0)
    }

    /// Generates a uniform random vector in the given range
    #[allow(dead_code)]
    pub fn vec2_range<R>(&mut self, xrange: R, yrange: R) -> Vec2
    where
        R: SampleRange<f32>,
    {
        Vec2::new(self.random_range(xrange), self.random_range(yrange))
    }

    /// Generates a uniform random direction vector
    #[allow(dead_code)]
    pub fn direction(&mut self) -> Vec2 {
        let theta = self.random() as f32 * std::f32::consts::PI * 2.0;
        Vec2::new(theta.cos(), theta.sin())
    }

    /// Generates a random value with the given normal distribution
    #[allow(dead_code)]
    pub fn normal<F>(&mut self, mean: F, std_dev: F) -> F
    where
        F: Float,
        StandardNormal: Distribution<F>,
    {
        Normal::new(mean, std_dev).unwrap().sample(&mut self.random)
    }

    /// Generates a random value with the given normal distribution
    /// Clamped to the given min / max
    #[allow(dead_code)]
    pub fn normal_clamped<F>(&mut self, mean: F, std_dev: F, min: F, max: F) -> F
    where
        F: Float,
        StandardNormal: Distribution<F>,
    {
        clampf(
            Normal::new(mean, std_dev).unwrap().sample(&mut self.random),
            min,
            max,
        )
    }
}

/// Perlin noies wrapper
pub struct PerlinNoise {
    perlin: Perlin,
}

impl Default for PerlinNoise {
    /// Constructs a default perlin noise function from the thread local rng
    fn default() -> Self {
        Self {
            perlin: Perlin::new().set_seed(random()),
        }
    }
}

impl PerlinNoise {
    /// Constructs a new perlin noies function from the given seed
    #[allow(dead_code)]
    pub fn new(seed: u32) -> Self {
        Self {
            perlin: Perlin::new().set_seed(seed),
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, point: f64, frequency: f64) -> f64 {
        self.get2d([point, 0.0], frequency)
    }

    #[allow(dead_code)]
    pub fn get2d(&self, point: [f64; 2], frequency: f64) -> f64 {
        self.perlin
            .get([point[0] * frequency, point[1] * frequency])
    }

    #[allow(dead_code)]
    pub fn get3d(&self, point: [f64; 3], frequency: f64) -> f64 {
        self.perlin.get([
            point[0] * frequency,
            point[1] * frequency,
            point[2] * frequency,
        ])
    }

    /// Generates a noisey vector using a random offset
    /// Untested
    #[allow(dead_code)]
    pub fn vec2(&self, random: &mut Random, point: f64, frequency: f64) -> Vec2 {
        let x = self.get(point, frequency) as f32;
        let y = self.get(point + random.random(), frequency) as f32;

        Vec2::new(x, y)
    }

    /// Generates a noisey random direction vector
    /// Untested
    #[allow(dead_code)]
    pub fn direction(&self, point: f64, frequency: f64) -> Vec2 {
        let theta = self.get(point, frequency) as f32;
        Vec2::new(theta.cos(), theta.sin())
    }
}
