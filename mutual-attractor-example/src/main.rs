use std::cell::RefCell;
use std::rc::Rc;

use glam::DVec2;
use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

const G: f64 = 1.0;

#[derive(Debug, Default)]
struct Mover {
    location: DVec2,
    velocity: DVec2,
    acceleration: DVec2,
    mass: f64,
}

impl Mover {
    fn new(mass: f64, x: f64, y: f64) -> Self {
        Self {
            location: DVec2::new(x, y),
            mass,
            ..Default::default()
        }
    }

    fn attract(&self, mover: &Mover) -> DVec2 {
        let force = self.location - mover.location;
        let distance = core::math::clampf(force.length(), 5.0, 20.0);
        let force = force.normalize_or_zero();
        let strength = (G * self.mass * mover.mass) / (distance * distance);

        force * strength
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
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        core::stroke_grayscale(screen, 0.0);
        core::fill_grayscale(screen, 0.0);

        core::shapes::ellipse(
            screen,
            self.location.x,
            self.location.y,
            self.mass * 16.0,
            self.mass * 16.0,
        )
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(
    screen: &mut Screen,
    dt: f64,
    mut movers: impl AsMut<[Mover]>,
) -> Result<(), ProcessingErr> {
    let movers = movers.as_mut();

    core::background_grayscale(screen, 255.0);

    for i in 0..movers.len() {
        for j in 0..movers.len() {
            if i != j {
                let f = movers[j].attract(&movers[i]);
                movers[i].apply_force(f);
            }
        }

        movers[i].update(dt);
        movers[i].display(screen)?;
    }

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let movers = Rc::new(RefCell::new(None));

    core::run(
        || {
            let mut rng = rand::thread_rng();

            let screen = setup()?;

            let mut mvrs = vec![];
            for _ in 0..10 {
                let x = rng.gen_range(0..screen.width()) as f64;
                let y = rng.gen_range(0..screen.height()) as f64;
                mvrs.push(Mover::new(rng.gen_range(0.1..2.0), x, y));
            }
            *movers.borrow_mut() = Some(mvrs);

            Ok(screen)
        },
        |screen, dt| draw(screen, dt, movers.borrow_mut().as_mut().unwrap()),
    )?;

    Ok(())
}
