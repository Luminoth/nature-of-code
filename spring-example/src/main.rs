use std::cell::RefCell;
use std::rc::Rc;

use glam::DVec2;
use processing::errors::ProcessingErr;
use processing::Screen;

const G: f64 = 1.0;

#[derive(Debug, Default)]
struct Spring {
    location: DVec2,
    len: f64,
    k: f64,
}

impl Spring {
    fn new(x: f64, y: f64, len: f64) -> Self {
        Self {
            location: DVec2::new(x, y),
            len,
            k: 0.01,
        }
    }

    fn attract(&self, mover: &Bob) -> DVec2 {
        /*let force = self.location - mover.location;
        let distance = core::math::clampf(force.length(), 5.0, 20.0);
        let force = force.normalize_or_zero();
        let strength = (G * self.mass * mover.mass) / (distance * distance);

        force * strength*/
        DVec2::default()
    }

    #[allow(dead_code)]
    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        // TODO:

        Ok(())
    }
}

#[derive(Debug, Default)]
struct Bob {
    location: DVec2,
    velocity: DVec2,
    acceleration: DVec2,
    mass: f64,
}

impl Bob {
    fn new(mass: f64) -> Self {
        Self {
            location: DVec2::default(),
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
        self.velocity += self.acceleration;
        self.location += self.velocity;

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
    bob: &mut Bob,
    spring: &mut Spring,
) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    spring.display(screen)?;

    let f = spring.attract(bob);
    bob.apply_force(f);

    bob.update(dt);
    bob.display(screen)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let bob = Rc::new(RefCell::new(None));
    let spring = Rc::new(RefCell::new(None));

    core::run(
        || {
            let screen = setup()?;

            *spring.borrow_mut() = Some(Spring::new(
                screen.width() as f64 / 2.0,
                10.0,
                screen.height() as f64 / 2.0,
            ));
            *bob.borrow_mut() = Some(Bob::new(10.0));

            Ok(screen)
        },
        |screen, dt| {
            draw(
                screen,
                dt,
                bob.borrow_mut().as_mut().unwrap(),
                spring.borrow_mut().as_mut().unwrap(),
            )
        },
    )?;

    Ok(())
}
