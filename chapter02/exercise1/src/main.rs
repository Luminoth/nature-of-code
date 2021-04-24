use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::Vector2;
use processing::errors::ProcessingErr;
use processing::Screen;

#[derive(Debug, Default)]
struct Balloon {
    location: Vector2<f64>,
    velocity: Vector2<f64>,
    acceleration: Vector2<f64>,
    topspeed: f64,
}

impl Balloon {
    fn new(screen: &Screen) -> Self {
        Self {
            location: Vector2::new(screen.width() as f64 / 2.0, screen.height() as f64),
            ..Default::default()
        }
    }

    fn check_edges(&mut self, screen: &Screen) {
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

    fn apply_force(&mut self, force: Vector2<f64>) {
        self.acceleration += force;
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.velocity.y = core::math::clampf(self.velocity.y, -1.0, 1.0);

        self.location += self.velocity;

        self.acceleration = Vector2::default();
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        core::stroke_grayscale(screen, 0.0);
        core::fill_grayscale(screen, 0.0);

        core::shapes::ellipse(screen, self.location.x, self.location.y, 16.0, 16.0)
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen, balloon: &mut Balloon) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    balloon.apply_force(Vector2::new(0.0, -0.005));

    balloon.apply_force(
        Vector2::new(
            core::math::map(core::sample_noise2d(), 0.0, 1.0, -1.0, 1.0),
            0.0,
        ) * 0.01,
    );

    balloon.update();
    balloon.check_edges(screen);
    balloon.display(screen)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let balloon = Rc::new(RefCell::new(None));

    core::run(
        || {
            let screen = setup()?;
            *balloon.borrow_mut() = Some(Balloon::new(&screen));
            Ok(screen)
        },
        |screen, _| draw(screen, balloon.borrow_mut().as_mut().unwrap()),
    )?;

    Ok(())
}
