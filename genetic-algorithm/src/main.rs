use std::cell::RefCell;
use std::rc::Rc;

use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

const TARGET: &str = "to be or not to be";

// key variables to tweak for a GA
// these values were chosen specifically
// to get a solution on average in 1000 generations
// larger populations will solve problems faster
// (1000 solves in 71 generations, 50,000 in 27 generations)
// time to solve can increase as iteration requirements go up
const MUTATION_RATE: f32 = 0.01; // 1% chance to mutate
const TOTAL_POPULATION: usize = 150;

#[derive(Debug, Copy, Clone)]
enum CrossoverMethod {
    #[allow(dead_code)]
    Midpoint,

    #[allow(dead_code)]
    Coin,
}

#[derive(Debug, Clone)]
struct Dna {
    target: &'static str,
    genes: Vec<char>,

    fitness: f32,
    generation: usize,
}

impl Dna {
    fn new(target: &'static str) -> Self {
        Self {
            target,
            genes: vec!['.'; target.len()],
            fitness: 0.0,
            generation: 0,
        }
    }

    fn random(target: &'static str) -> Self {
        let mut rng = rand::thread_rng();

        let mut genes = Vec::with_capacity(target.len());
        for _ in 0..genes.capacity() {
            genes.push(rng.gen_range(32..128).into());
        }

        Self {
            target,
            genes,
            fitness: 0.0,
            generation: 0,
        }
    }

    fn phrase(&self) -> String {
        self.genes.iter().collect()
    }

    fn fitness(&mut self) {
        let mut score = 0;
        for i in 0..self.genes.len() {
            if self.genes[i] == self.target.chars().nth(i).unwrap() {
                score += 1;
            }
        }

        // exponential fitness score
        self.fitness = (score * score) as f32;

        // normalize to 0..1 for the mating pool
        self.fitness /= (self.target.len() * self.target.len()) as f32;

        assert!(self.fitness <= 1.0);
    }

    fn crossover(&self, partner: &Dna, method: CrossoverMethod) -> Dna {
        let mut rng = rand::thread_rng();

        let mut child = Dna::new(TARGET);

        match method {
            CrossoverMethod::Midpoint => {
                let midpoint = rng.gen_range(0..self.genes.len());
                for i in 0..self.genes.len() {
                    child.genes[i] = if i > midpoint {
                        self.genes[i]
                    } else {
                        partner.genes[i]
                    };
                }
            }
            CrossoverMethod::Coin => {
                for i in 0..self.genes.len() {
                    let coin = rng.gen_range(0..=1);
                    child.genes[i] = if coin == 0 {
                        self.genes[i]
                    } else {
                        partner.genes[i]
                    };
                }
            }
        }

        child.generation = self.generation + 1;

        child
    }

    fn mutate(&mut self, mutation_rate: f32) {
        let mut rng = rand::thread_rng();

        for i in 0..self.genes.len() {
            if rng.gen_range(0.0..1.0) < mutation_rate {
                //println!("mutation!");
                self.genes[i] = rng.gen_range(32..128).into();
            }
        }
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen, _: f64, population: &mut Vec<Dna>) -> Result<(), ProcessingErr> {
    let mut rng = rand::thread_rng();

    core::background_grayscale(screen, 255.0);

    let mut mating_pool = vec![];

    // wheel of fortune probability
    // have to clone parents here
    // (tho probably not as much as we are)
    // since we overwrite their population entries
    // when they reproduce
    for dna in population.iter_mut() {
        dna.fitness();

        let n = (dna.fitness * 100.0) as usize;
        for _ in 0..n {
            mating_pool.push(dna.clone());
        }
    }

    if mating_pool.len() == 0 {
        println!("population unfit for mating!");
        return Ok(());
    }

    for dna in population.iter_mut() {
        // select the parents
        let a = rng.gen_range(0..mating_pool.len());
        let mut b = rng.gen_range(0..mating_pool.len());
        while a == b {
            b = rng.gen_range(0..mating_pool.len());
        }

        let parent_a = &mating_pool[a];
        let parent_b = &mating_pool[b];

        let mut child = parent_a.crossover(parent_b, CrossoverMethod::Coin);
        child.mutate(MUTATION_RATE);

        *dna = child;

        // calculate the new fitness for the best phrase check
        dna.fitness();
    }

    let mut best_phrase = 0;
    for i in 1..population.len() {
        if population[i].fitness > population[best_phrase].fitness {
            best_phrase = i;
        }
    }

    let fittest = &population[best_phrase];

    core::text(
        screen,
        format!("Best phrase: {}", fittest.phrase()),
        0.0,
        0.0,
    )?;

    if (fittest.fitness - 1.0).abs() < 0.01 {
        println!("Generated target in {} generations", fittest.generation);
        std::process::exit(0);
    }

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let population = Rc::new(RefCell::new(None));

    core::run(
        || {
            let screen = setup()?;

            let mut p: Vec<Dna> = vec![];

            for _ in 0..TOTAL_POPULATION {
                p.push(Dna::random(TARGET));
            }

            *population.borrow_mut() = Some(p);

            Ok(screen)
        },
        |screen, dt| draw(screen, dt, population.borrow_mut().as_mut().unwrap()),
    )?;

    Ok(())
}
