use std::cell::RefCell;
use std::rc::Rc;

use processing::errors::ProcessingErr;
use processing::Screen;

#[derive(Debug)]
struct Wave {
    angle: f64,
    angular_velocity: f64,
}

impl Default for Wave {
    fn default() -> Self {
        Self {
            angle: 0.0,
            angular_velocity: 0.1,
        }
    }
}

impl Wave {
    fn update(&mut self, dt: f64) {
        self.angle += dt;
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        let mut angle = self.angle;

        for x in (0..screen.width()).step_by(24) {
            let y = angle.sin();
            //let y = core::noise(angle, 1.0); // exercise 3.9
            let y = core::math::map(y, -1.0, 1.0, 0.0, screen.height() as f64);

            core::stroke_grayscale(screen, 0.0);
            core::fill_grayscale_alpha(screen, 0.0, 50.0);

            core::shapes::ellipse(screen, x as f64, y, 48.0, 48.0)?;

            angle += self.angular_velocity;
        }

        Ok(())
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(400, 200)
}

fn draw(screen: &mut Screen, mut waves: impl AsMut<[Wave]>) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    for wave in waves.as_mut() {
        wave.update(0.02);
        wave.display(screen)?;
    }

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let waves = Rc::new(RefCell::new(None));

    core::run(
        || {
            let screen = setup()?;

            let w = vec![
                Wave {
                    angle: 2.0,
                    angular_velocity: 0.2,
                },
                Wave::default(),
                Wave {
                    angle: 5.0,
                    angular_velocity: 0.3,
                },
            ];
            *waves.borrow_mut() = Some(w);

            Ok(screen)
        },
        |screen, _| draw(screen, waves.borrow_mut().as_mut().unwrap()),
    )?;

    Ok(())
}
