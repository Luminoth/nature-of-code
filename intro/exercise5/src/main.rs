use std::cell::RefCell;
use std::rc::Rc;

use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;
use rand_distr::{Distribution, Normal};

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

    fn step(&mut self, screen: &Screen) {
        let mut rng = rand::thread_rng();

        let normal_x = Normal::new(5.0, 2.0).unwrap();
        let normal_y = Normal::new(5.0, 2.0).unwrap();

        let stepx = normal_x.sample(&mut rng) as i32;
        let stepy = normal_y.sample(&mut rng) as i32;

        let dirx: i32 = rng.gen_range(0..3) - 1;
        let diry: i32 = rng.gen_range(0..3) - 1;

        self.x = core::math::clamp(self.x + dirx.signum() * stepx, 0, screen.width() as i32);
        self.y = core::math::clamp(self.y + diry.signum() * stepy, 0, screen.height() as i32);
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    let mut screen = core::create_canvas(640, 360)?;
    core::background_grayscale(&mut screen, 255.0);

    Ok(screen)
}

fn draw(screen: &mut Screen, walker: &mut Walker) -> Result<(), ProcessingErr> {
    walker.step(screen);
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
