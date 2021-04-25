use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::Vector2;
use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

#[derive(Debug, Default)]
struct Mover {
    location: Vector2<f64>,
    velocity: Vector2<f64>,
    acceleration: Vector2<f64>,
    mass: f64,
    topspeed: f64,
}

impl Mover {
    fn new(screen: &Screen) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..screen.width()) as f64;
        let y = rng.gen_range(0..screen.height()) as f64;

        Self {
            location: Vector2::new(x, y),
            mass: 1.0,
            topspeed: 1.0,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    fn wrap_edges(&mut self, screen: &Screen) {
        if self.location.x > screen.width() as f64 {
            self.location.x = 0.0;
        } else if self.location.x < 0.0 {
            self.location.x = screen.width() as f64;
        }

        if self.location.y > screen.height() as f64 {
            self.location.y = 0.0;
        } else if self.location.y < 0.0 {
            self.location.y = screen.height() as f64;
        }
    }

    #[allow(dead_code)]
    fn stop_edges(&mut self, screen: &Screen) {
        if self.location.x > screen.width() as f64 {
            self.location.x = screen.width() as f64;
        } else if self.location.x < 0.0 {
            self.location.x = 0.0;
        }

        if self.location.y > screen.height() as f64 {
            self.location.y = screen.height() as f64;
        } else if self.location.y < 0.0 {
            self.location.y = 0.0;
        }
    }

    #[allow(dead_code)]
    fn bounce_edges(&mut self, screen: &Screen) {
        if self.location.x > screen.width() as f64 {
            self.location.x = screen.width() as f64;
            self.velocity.x *= -1.0;
        } else if self.location.x < 0.0 {
            self.location.x = 0.0;
            self.velocity.x *= -1.0;
        }

        if self.location.y > screen.height() as f64 {
            self.location.y = screen.height() as f64;
            self.velocity.y *= -1.0;
        } else if self.location.y < 0.0 {
            self.location.y = 0.0;
            self.velocity.y *= -1.0;
        }
    }

    fn apply_force(&mut self, force: Vector2<f64>) {
        let force = force / self.mass;
        self.acceleration += force;
    }

    fn update(&mut self) {
        let mut rng = rand::thread_rng();

        self.acceleration =
            core::math::vector2_random() * core::sample_noise2d() * rng.gen_range(0.1..0.5);

        self.velocity = (self.velocity + self.acceleration).cap_magnitude(self.topspeed);
        self.location += self.velocity;

        self.acceleration = Vector2::default();
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

fn draw(screen: &mut Screen, mover: &mut Mover) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    mover.update();
    mover.bounce_edges(screen);
    mover.display(screen)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let mover = Rc::new(RefCell::new(None));

    core::run(
        || {
            let screen = setup()?;
            *mover.borrow_mut() = Some(Mover::new(&screen));
            Ok(screen)
        },
        |screen, _| draw(screen, mover.borrow_mut().as_mut().unwrap()),
    )?;

    Ok(())
}
