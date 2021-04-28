//! ECS resources

pub mod debug;

use bevy::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use num_traits::Float;
use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::prelude::*;
use rand_distr::{Normal, StandardNormal};

use crate::clampf;

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

    /// Generates a uniform random direction vector, never 0 length
    #[allow(dead_code)]
    pub fn direction(&mut self) -> Vec2 {
        let mut direction = (self.vec2() * 2.0 - Vec2::new(1.0, 1.0)).normalize();
        while !direction.is_finite() {
            direction = (self.vec2() * 2.0 - Vec2::new(1.0, 1.0)).normalize();
        }
        direction
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

    pub fn get(&self, point: f64, frequency: f64) -> f64 {
        self.perlin.get([point * frequency, 0.0])
    }

    /*/// Sample noise in the domain [0..1),[0..1) scaled by frequency
    /// Result is in the range (-1..1)
    #[allow(dead_code)]
    pub fn sample(&self, random: &mut Random, frequency: f64) -> f64 {
        self.perlin.get([
            random.random_range(0.0..1.0) * frequency,
            random.random_range(0.0..1.0) * frequency,
        ])
    }*/

    /*/// Generates a noisey vector in the range ((-1..1), (-1..1))
    #[allow(dead_code)]
    pub fn vec2(&self, random: &mut Random, frequency: f64) -> Vec2 {
        Vec2::new(
            self.sample(random, frequency) as f32,
            self.sample(random, frequency) as f32,
        )
    }*/

    /*/// Generates a uniform random direction vector, never 0 length
    #[allow(dead_code)]
    pub fn direction(&self, random: &mut Random, frequency: f64) -> Vec2 {
        let mut direction = self.vec2(random, frequency).normalize();
        while !direction.is_finite() {
            direction = self.vec2(random, frequency).normalize();
        }
        direction
    }*/
}
