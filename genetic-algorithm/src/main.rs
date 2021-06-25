use std::cell::RefCell;
use std::rc::Rc;

use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

const TARGET: &str = "to be or not to be";
const MUTATION_RATE: f32 = 0.01;
const TOTAL_POPULATION: usize = 150;

#[derive(Debug, Default, Clone)]
struct Dna {
    // 18 genes for "to be or not to be"
    genes: [char; 18],

    fitness: f32,
    generation: usize,
}

impl Dna {
    fn new() -> Self {
        let mut rng = rand::thread_rng();

        let mut genes = ['.'; 18];
        for gene in genes.iter_mut() {
            *gene = rng.gen_range(32..128).into();
        }

        Self {
            genes,
            ..Default::default()
        }
    }

    fn phrase(&self) -> String {
        self.genes.iter().collect()
    }

    fn fitness(&mut self, target: impl AsRef<str>) {
        let target = target.as_ref();

        let mut score = 0;
        for i in 0..self.genes.len() {
            if self.genes[i] == target.chars().nth(i).unwrap() {
                score += 1;
            }
        }
        self.fitness = score as f32 / target.len() as f32;
    }

    fn crossover(&self, partner: &Dna) -> Dna {
        let mut rng = rand::thread_rng();

        let mut child = Dna::default();

        // random midpoint method
        /*let midpoint = rng.gen_range(0..self.genes.len());
        for i in 0..self.genes.len() {
            child.genes[i] = if i > midpoint {
                self.genes[i]
            } else {
                partner.genes[i]
            };
        }*/

        // coin method
        for i in 0..self.genes.len() {
            let coin = rng.gen_range(0..=1);
            child.genes[i] = if coin == 0 {
                self.genes[i]
            } else {
                partner.genes[i]
            };
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
        dna.fitness(TARGET);

        let n = (dna.fitness * 100.0) as usize;
        for _ in 0..n {
            mating_pool.push(dna.clone());
        }
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

        let mut child = parent_a.crossover(parent_b);
        child.mutate(MUTATION_RATE);

        *dna = child;

        // calculate the new fitness for the best phrase check
        dna.fitness(TARGET);
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

    if fittest.fitness == 1.0 {
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
                p.push(Dna::new());
            }

            *population.borrow_mut() = Some(p);

            Ok(screen)
        },
        |screen, dt| draw(screen, dt, population.borrow_mut().as_mut().unwrap()),
    )?;

    Ok(())
}
