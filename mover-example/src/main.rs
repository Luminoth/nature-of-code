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
}

impl Mover {
    fn new(screen: &Screen) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            location: Vector2::new(
                rng.gen_range(0..screen.width()) as f64,
                rng.gen_range(0..screen.height()) as f64,
            ),
            velocity: Vector2::new(rng.gen_range(-2.0..2.0), rng.gen_range(-2.0..2.0)),
        }
    }

    fn check_edges(&mut self, screen: &Screen) {
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

    fn update(&mut self) {
        self.location += self.velocity;
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

fn draw(screen: &mut Screen, mover: &mut Mover) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    mover.update();
    mover.check_edges(screen);
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
        |screen| draw(screen, mover.borrow_mut().as_mut().unwrap()),
    )?;

    Ok(())
}
