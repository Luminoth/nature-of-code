use std::cell::RefCell;
use std::rc::Rc;

use glam::DVec2;
use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

const MUTATION_RATE: f32 = 0.01; // 1% chance to mutate
const TOTAL_POPULATION: usize = 50;
const LIFETIME: usize = 500;

#[derive(Debug, Copy, Clone)]
enum CrossoverMethod {
    #[allow(dead_code)]
    Midpoint,

    #[allow(dead_code)]
    Coin,
}

#[derive(Debug, Clone)]
struct Dna {
    genes: Vec<DVec2>,

    max_force: f64,

    fitness: f64,
}

impl Dna {
    fn random_gene(max_force: f64) -> DVec2 {
        let mut rng = rand::thread_rng();

        core::math::vector2_random_angle() * rng.gen_range(0.0..max_force)
    }

    fn new(lifetime: usize) -> Self {
        Self {
            genes: vec![DVec2::default(); lifetime],
            max_force: 0.1,
            fitness: 0.0,
        }
    }

    fn random(lifetime: usize) -> Self {
        let max_force = 0.1;

        let mut genes = Vec::with_capacity(lifetime);
        for _ in 0..genes.capacity() {
            genes.push(Dna::random_gene(max_force));
        }

        Self {
            genes,
            max_force,
            fitness: 0.0,
        }
    }

    fn fitness(&mut self, location: DVec2, target: DVec2) {
        let d = location.distance_squared(target);
        self.fitness = (1.0 / d).powf(2.0);
    }

    fn crossover(&self, partner: &Dna, method: CrossoverMethod) -> Dna {
        let mut rng = rand::thread_rng();

        assert_eq!(self.genes.len(), partner.genes.len());
        let mut child = Dna::new(self.genes.len());

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

        child
    }

    fn mutate(&mut self, mutation_rate: f32) {
        let mut rng = rand::thread_rng();

        for i in 0..self.genes.len() {
            if rng.gen_range(0.0..1.0) < mutation_rate {
                //println!("mutation!");
                self.genes[i] = Dna::random_gene(self.max_force);
            }
        }
    }
}

#[derive(Debug)]
struct Rocket {
    location: DVec2,
    velocity: DVec2,
    acceleration: DVec2,

    r: f64,

    dna: Dna,
    gene_counter: usize,
}

impl Rocket {
    fn new(location: DVec2, lifetime: usize) -> Self {
        Self {
            location,
            velocity: DVec2::default(),
            acceleration: DVec2::default(),

            r: 4.0,

            dna: Dna::random(lifetime),
            gene_counter: 0,
        }
    }

    fn heading(&self) -> f64 {
        self.velocity.x.atan2(self.velocity.y)
    }

    fn apply_force(&mut self, force: DVec2) {
        self.acceleration += force;
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.location += self.velocity;

        self.acceleration = DVec2::default();
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        let theta = self.heading() + std::f64::consts::FRAC_PI_2;

        core::stroke_grayscale(screen, 0.0);
        core::fill_grayscale_alpha(screen, 200.0, 100.0);

        screen.push_matrix();

        core::translate(screen, self.location.x, self.location.y);
        core::rotate(screen, theta);

        // thrusters
        screen.rect_mode(&core::shapes::RectMode::Center.to_string());
        core::fill_grayscale(screen, 0.0);
        core::shapes::rect(screen, -self.r / 2.0, self.r * 2.0, self.r / 2.0, self.r)?;
        core::shapes::rect(screen, self.r / 2.0, self.r * 2.0, self.r / 2.0, self.r)?;

        // rocket body
        core::fill_grayscale(screen, 175.0);

        // TODO: beginShape(TRIANGLES);

        // TODO: convert to world coords
        //vertex(0.0, -self.r * 2.0);
        //vertex(-self., self.r * 2.0);
        //vertex(self.r, self.r * 2.0);

        // TODO: endShape();

        screen.pop_matrix();

        Ok(())
    }

    fn run(&mut self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        self.apply_force(self.dna.genes[self.gene_counter]);
        self.gene_counter = (self.gene_counter + 1) % self.dna.genes.len();

        self.update();

        self.display(screen)?;

        Ok(())
    }
}

struct Population {
    mutation_rate: f32,

    population: Vec<Rocket>,
    mating_pool: Vec<Dna>,
    generations: usize,

    target: DVec2,
    lifetime: usize,
    life_counter: usize,
}

impl Population {
    fn new(
        location: DVec2,
        mutation_rate: f32,
        total_population: usize,
        target: DVec2,
        lifetime: usize,
    ) -> Self {
        let mut population = vec![];
        for _ in 0..total_population {
            population.push(Rocket::new(location, lifetime));
        }

        Self {
            mutation_rate,
            population,
            mating_pool: vec![],
            generations: 0,
            target,
            lifetime,
            life_counter: 0,
        }
    }

    fn fitness(&mut self) {
        for member in self.population.iter_mut() {
            member.dna.fitness(member.location, self.target);
        }
    }

    fn selection(&mut self) {
        self.mating_pool.clear();

        // wheel of fortune probability
        // have to clone parents here
        // (tho probably not as much as we are)
        // since we overwrite their population entries
        // when they reproduce
        for member in self.population.iter() {
            let n = (member.dna.fitness * 100.0) as usize;
            for _ in 0..n {
                self.mating_pool.push(member.dna.clone());
            }
        }
    }

    fn reproduction(&mut self) {
        if self.mating_pool.len() == 0 {
            println!("population unfit for mating!");
            return;
        }

        let mut rng = rand::thread_rng();

        for member in self.population.iter_mut() {
            // select the parents
            let a = rng.gen_range(0..self.mating_pool.len());
            let mut b = rng.gen_range(0..self.mating_pool.len());
            while a == b {
                b = rng.gen_range(0..self.mating_pool.len());
            }

            let parent_a = &self.mating_pool[a];
            let parent_b = &self.mating_pool[b];

            let mut child = parent_a.crossover(&parent_b, CrossoverMethod::Coin);
            child.mutate(self.mutation_rate);

            member.dna = child;
        }

        self.generations += 1;
    }

    fn live(&mut self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        for member in self.population.iter_mut() {
            member.run(screen)?;
        }

        Ok(())
    }

    fn run(&mut self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        // target
        core::fill_grayscale(screen, 0.0);
        core::shapes::ellipse(screen, self.target.x, self.target.y, 24.0, 24.0)?;

        if self.life_counter < self.lifetime {
            self.live(screen)?;
            self.life_counter += 1;
        } else {
            self.life_counter = 0;

            self.fitness();
            self.selection();
            self.reproduction();
        }

        Ok(())
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen, _: f64, population: &mut Population) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    population.run(screen)?;

    /*let mut best_phrase = 0;
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
    }*/

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let population = Rc::new(RefCell::new(None));

    core::run(
        || {
            let screen = setup()?;

            let target = DVec2::new(screen.width() as f64 / 2.0, 20.0);

            *population.borrow_mut() = Some(Population::new(
                DVec2::new(screen.width() as f64 / 2.0, screen.height() as f64 / 2.0),
                MUTATION_RATE,
                TOTAL_POPULATION,
                target,
                LIFETIME,
            ));

            Ok(screen)
        },
        |screen, dt| draw(screen, dt, population.borrow_mut().as_mut().unwrap()),
    )?;

    Ok(())
}
