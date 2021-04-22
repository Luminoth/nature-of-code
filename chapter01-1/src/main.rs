use std::cell::RefCell;
use std::rc::Rc;

use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

const EXERCISE_1_1: bool = true;

#[derive(Debug, Default)]
struct Walker {
    x: i32,
    y: i32,
}

impl Walker {
    fn new(screen: &mut Screen) -> Self {
        Self {
            x: (screen.width() / 2) as i32,
            y: (screen.height() / 2) as i32,
        }
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        core::stroke_grayscale(screen, 0.0);
        core::shapes::point(screen, self.x as f64, self.y as f64)
    }

    fn step(&mut self) {
        let mut rng = rand::thread_rng();

        let (stepx, stepy) = if EXERCISE_1_1 {
            // bias down (+y) and to the right (+x)
            (
                rng.gen_range::<i32, _>(-1..3),
                rng.gen_range::<i32, _>(-1..3),
            )
        } else {
            (rng.gen_range(0..3) - 1, rng.gen_range(0..3) - 1)
        };

        // use signum() so we only step 1 at a time
        self.x += stepx.signum();
        self.y += stepy.signum();
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    let mut screen = core::create_canvas(640, 360)?;
    core::background_grayscale(&mut screen, 255.0);

    Ok(screen)
}

fn draw(screen: &mut Screen, walker: &mut Walker) -> Result<(), ProcessingErr> {
    walker.step();
    walker.display(screen)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let walker = Rc::new(RefCell::new(None));

    core::run(
        || {
            let mut screen = setup()?;
            *walker.borrow_mut() = Some(Walker::new(&mut screen));
            Ok(screen)
        },
        |screen| draw(screen, walker.borrow_mut().as_mut().unwrap()),
    )?;

    Ok(())
}
