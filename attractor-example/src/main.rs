use std::cell::RefCell;
use std::rc::Rc;

use glam::DVec2;
use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

const G: f64 = 1.0;

#[derive(Debug, Default)]
struct Attractor {
    location: DVec2,
    mass: f64,
}

impl Attractor {
    fn new(mass: f64, x: f64, y: f64) -> Self {
        Self {
            location: DVec2::new(x, y),
            mass,
        }
    }

    fn attract(&self, mover: &mut Mover) -> DVec2 {
        let force = self.location - mover.location;
        let distance = core::math::clampf(force.length(), 5.0, 20.0);
        let force = force.normalize_or_zero();
        let strength = (G * self.mass * mover.mass) / (distance * distance);

        force * strength
    }

    #[allow(dead_code)]
    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        core::stroke_grayscale(screen, 0.0);
        core::fill_grayscale_alpha(screen, 175.0, 200.0);

        core::shapes::ellipse(
            screen,
            self.location.x,
            self.location.y,
            self.mass * 2.0,
            self.mass * 2.0,
        )
    }
}

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
    attractors: impl AsRef<[Attractor]>,
) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    /*for attractor in attractors.as_ref() {
        attractor.display(screen)?;
    }*/

    for mover in movers.as_mut() {
        for attractor in attractors.as_ref() {
            let f = attractor.attract(mover);
            mover.apply_force(f);
        }

        mover.update(dt);
        mover.display(screen)?;
    }

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let movers = Rc::new(RefCell::new(None));
    let attractors = Rc::new(RefCell::new(None));

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

            let mut attrs = vec![];
            for _ in 0..10 {
                let x = rng.gen_range(0..screen.width()) as f64;
                let y = rng.gen_range(0..screen.height()) as f64;
                attrs.push(Attractor::new(rng.gen_range(10.0..20.0), x, y));
            }
            *attractors.borrow_mut() = Some(attrs);

            Ok(screen)
        },
        |screen, dt| {
            draw(
                screen,
                dt,
                movers.borrow_mut().as_mut().unwrap(),
                attractors.borrow().as_ref().unwrap(),
            )
        },
    )?;

    Ok(())
}
