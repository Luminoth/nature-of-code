//! ECS resources

pub mod creatures;

//use bevy::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use rand::{random, Rng};

pub struct Noise {
    perlin: Perlin,
}

impl Noise {
    pub fn get_perlin(&self) -> f64 {
        let mut rng = rand::thread_rng();
        self.perlin
            .get([rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)])
    }

    // TODO: does this produce negative directions?
    /*pub fn vector2(&self) -> Vec3 {
        Vec3::new(self.get_perlin() as f32, self.get_perlin() as f32, 0.0).normalize()
    }*/
}

impl Default for Noise {
    fn default() -> Self {
        Self {
            perlin: Perlin::new().set_seed(random()),
        }
    }
}
