use std::cell::RefCell;
use std::rc::Rc;

use glam::DVec2;
use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

#[derive(Debug, Default)]
struct Particle {
    location: DVec2,
    velocity: DVec2,
    acceleration: DVec2,

    lifespan: f64,
}

impl Particle {
    fn new(location: DVec2) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            location,
            acceleration: DVec2::new(0.0, 0.05),
            velocity: DVec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-2.0..0.0)),
            lifespan: 255.0,
        }
    }

    fn is_dead(&self) -> bool {
        self.lifespan < 0.0
    }

    fn update(&mut self, _dt: f64) {
        self.velocity += self.acceleration; // * dt;
        self.location += self.velocity; // * dt;

        //self.acceleration = DVec2::default();

        self.lifespan -= 2.0;
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        core::stroke_grayscale_alpha(screen, 0.0, self.lifespan as f32);
        core::fill_grayscale_alpha(screen, 0.0, self.lifespan as f32);

        core::shapes::ellipse(screen, self.location.x, self.location.y, 8.0, 8.0)
    }

    fn run(&mut self, screen: &mut Screen, dt: f64) -> Result<(), ProcessingErr> {
        self.update(dt);
        self.display(screen)?;

        Ok(())
    }
}

#[derive(Debug, Default)]
struct ParticleSystem {
    origin: DVec2,
    particles: Vec<Particle>,
}

impl ParticleSystem {
    fn new(x: f64, y: f64) -> Self {
        Self {
            origin: DVec2::new(x, y),
            ..Default::default()
        }
    }

    fn add_particle(&mut self) {
        self.particles.push(Particle::new(self.origin));
    }

    fn run(&mut self, screen: &mut Screen, dt: f64) -> Result<(), ProcessingErr> {
        self.add_particle();

        // drain_filter() equivalent
        let mut i = 0;
        while i != self.particles.len() {
            let particle = &mut self.particles[i];
            particle.run(screen, dt)?;

            if particle.is_dead() {
                self.particles.remove(i);
            } else {
                i += 1;
            }
        }

        Ok(())
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(
    screen: &mut Screen,
    dt: f64,
    particle_system: &mut ParticleSystem,
) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    particle_system.run(screen, dt)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let particle_system = Rc::new(RefCell::new(None));

    core::run(
        || {
            let screen = setup()?;

            let mut p = ParticleSystem::new(screen.width() as f64 / 2.0, 50.0);
            for _ in 0..10 {
                p.add_particle();
            }
            *particle_system.borrow_mut() = Some(p);

            Ok(screen)
        },
        |screen, dt| draw(screen, dt, particle_system.borrow_mut().as_mut().unwrap()),
    )?;

    Ok(())
}
