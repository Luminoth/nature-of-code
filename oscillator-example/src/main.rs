use std::cell::RefCell;
use std::rc::Rc;

use glam::DVec2;
use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

#[derive(Debug, Default)]
struct Oscillator {
    angle: DVec2,
    velocity: DVec2,
    amplitude: DVec2,
}

impl Oscillator {
    fn random(screen: &Screen) -> Self {
        let mut rand = rand::thread_rng();

        Self {
            angle: DVec2::default(),
            velocity: DVec2::new(rand.gen_range(-0.05..0.05), rand.gen_range(-0.05..0.05)),
            amplitude: DVec2::new(screen.width() as f64 / 2.0, screen.height() as f64 / 2.0),
        }
    }

    fn oscillate(&mut self) {
        self.angle += self.velocity;
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        let x = self.angle.x.sin() * self.amplitude.x;
        let y = self.angle.y.sin() * self.amplitude.y;

        screen.push_matrix();

        core::translate(
            screen,
            screen.width() as f64 / 2.0,
            screen.height() as f64 / 2.0,
        );

        core::stroke_grayscale(screen, 0.0);
        core::fill_grayscale(screen, 175.0);
        core::shapes::line(screen, 0.0, 0.0, x, y)?;
        core::shapes::ellipse(screen, x, y, 16.0, 16.0)?;

        screen.pop_matrix();

        Ok(())
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen, oscillator: &mut Oscillator) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    oscillator.oscillate();
    oscillator.display(screen)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let oscillator = Rc::new(RefCell::new(None));

    core::run(
        || {
            let screen = setup()?;
            *oscillator.borrow_mut() = Some(Oscillator::random(&screen));
            Ok(screen)
        },
        |screen, _| draw(screen, oscillator.borrow_mut().as_mut().unwrap()),
    )?;

    Ok(())
}
