use std::cell::RefCell;
use std::rc::Rc;

use glam::DVec2;
use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

// design decision - using a tagged enum type
// instead of a Trait to avoid having to Box individual particles

#[derive(Debug, Default)]
struct Repeller {
    location: DVec2,
    r: f64,
    strength: f64,
}

impl Repeller {
    fn new(x: f64, y: f64) -> Self {
        Self {
            location: DVec2::new(x, y),
            r: 10.0,
            strength: 100.0,
        }
    }

    fn repel(&self, particle: &Particle) -> DVec2 {
        let dir = self.location - particle.core.location;
        let d = core::math::clampf(dir.length(), 5.0, 100.0);
        let dir = dir.normalize_or_zero();
        let force = -1.0 * self.strength / (d * d);

        dir * force
    }

    #[allow(dead_code)]
    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        core::stroke_grayscale(screen, 255.0);
        core::fill_grayscale(screen, 255.0);

        core::shapes::ellipse(
            screen,
            self.location.x,
            self.location.y,
            self.r * 2.0,
            self.r * 2.0,
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ParticleType {
    Basic,
    Confetti,
}

impl ParticleType {
    fn display(&self, screen: &mut Screen, core: &ParticleCore) -> Result<(), ProcessingErr> {
        screen.push_matrix();

        core::translate(screen, core.location.x, core.location.y);

        match self {
            ParticleType::Basic => {
                core::stroke_grayscale_alpha(screen, 0.0, core.lifespan as f32);
                core::fill_grayscale_alpha(screen, 0.0, core.lifespan as f32);

                core::shapes::ellipse(screen, 0.0, 0.0, 8.0, 8.0)?;
            }
            ParticleType::Confetti => {
                let _theta = core::math::map(
                    core.location.x,
                    0.0,
                    screen.width() as f64,
                    0.0,
                    4.0 * std::f64::consts::PI,
                );

                core::stroke_grayscale_alpha(screen, 0.0, core.lifespan as f32);
                core::fill_grayscale_alpha(screen, 175.0, core.lifespan as f32);

                //core::rotate(screen, theta);

                screen.rect_mode(&core::shapes::RectMode::Center.to_string());
                core::shapes::rect(screen, 0.0, 0.0, 8.0, 8.0)?;
            }
        }

        screen.pop_matrix();

        Ok(())
    }
}

#[derive(Debug, Default)]
struct ParticleCore {
    location: DVec2,
    velocity: DVec2,
    acceleration: DVec2,
    mass: f64,

    lifespan: f64,
}

impl ParticleCore {
    fn new(location: DVec2) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            location,
            velocity: DVec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-2.0..0.0)),
            mass: 1.0,
            lifespan: 255.0,
            ..Default::default()
        }
    }

    fn is_dead(&self) -> bool {
        self.lifespan < 0.0
    }

    fn apply_force(&mut self, force: DVec2) {
        let force = if self.mass > 0.0 {
            force / self.mass
        } else {
            force
        };
        self.acceleration += force;
    }

    fn update(&mut self, _dt: f64) {
        self.velocity += self.acceleration; // * dt;
        self.location += self.velocity; // * dt;

        self.acceleration = DVec2::default();

        self.lifespan -= 2.0;
    }
}

#[derive(Debug)]
struct Particle {
    core: ParticleCore,
    r#type: ParticleType,
}

impl Particle {
    fn basic(location: DVec2) -> Self {
        Self {
            core: ParticleCore::new(location),
            r#type: ParticleType::Basic,
        }
    }

    fn confetti(location: DVec2) -> Self {
        Self {
            core: ParticleCore::new(location),
            r#type: ParticleType::Confetti,
        }
    }

    fn is_dead(&self) -> bool {
        self.core.is_dead()
    }

    fn apply_force(&mut self, force: DVec2) {
        self.core.apply_force(force)
    }

    fn update(&mut self, dt: f64) {
        self.core.update(dt);
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        self.r#type.display(screen, &self.core)?;

        Ok(())
    }

    fn run(&mut self, screen: &mut Screen, dt: f64) -> Result<(), ProcessingErr> {
        self.update(dt);
        self.display(screen)?;

        Ok(())
    }
}

#[derive(Default)]
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
        let mut rng = rand::thread_rng();

        let c = rng.gen_range(0.0..1.0);
        if c < 0.5 {
            self.particles.push(Particle::basic(self.origin));
        } else {
            self.particles.push(Particle::confetti(self.origin));
        }
    }

    fn apply_force(&mut self, force: DVec2) {
        for particle in self.particles.iter_mut() {
            particle.apply_force(force);
        }
    }

    fn apply_repeller(&mut self, repeller: &Repeller) {
        for particle in self.particles.iter_mut() {
            let force = repeller.repel(particle);
            particle.apply_force(force);
        }
    }

    fn run(&mut self, screen: &mut Screen, dt: f64) -> Result<(), ProcessingErr> {
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
    repeller: &Repeller,
) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 100.0);

    particle_system.add_particle();

    let gravity = DVec2::new(0.0, 0.1);
    particle_system.apply_force(gravity);

    particle_system.apply_repeller(repeller);

    particle_system.run(screen, dt)?;
    repeller.display(screen)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let particle_system = Rc::new(RefCell::new(None));
    let repeller = Rc::new(RefCell::new(None));

    core::run(
        || {
            let screen = setup()?;

            *particle_system.borrow_mut() =
                Some(ParticleSystem::new(screen.width() as f64 / 2.0, 50.0));

            *repeller.borrow_mut() = Some(Repeller::new(
                screen.width() as f64 / 2.0 - 20.0,
                screen.height() as f64 / 2.0,
            ));

            Ok(screen)
        },
        |screen, dt| {
            draw(
                screen,
                dt,
                particle_system.borrow_mut().as_mut().unwrap(),
                repeller.borrow().as_ref().unwrap(),
            )
        },
    )?;

    Ok(())
}
