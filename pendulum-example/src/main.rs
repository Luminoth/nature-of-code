use std::cell::RefCell;
use std::rc::Rc;

use glam::DVec2;
use processing::errors::ProcessingErr;
use processing::Screen;

#[derive(Debug)]
struct Pendulum {
    origin: DVec2,

    r: f64,
    angle: f64,

    angular_acceleration: f64,
    angular_velocity: f64,
}

impl Default for Pendulum {
    fn default() -> Self {
        Self {
            origin: DVec2::default(),
            r: 1.0,
            angle: std::f64::consts::FRAC_PI_4,
            angular_acceleration: 0.0,
            angular_velocity: 0.0,
        }
    }
}

impl Pendulum {
    fn new(origin: DVec2, r: f64) -> Self {
        Self {
            origin,
            r,
            ..Default::default()
        }
    }

    fn update(&mut self, g: f64) {
        self.angular_acceleration = (-g * self.angle.sin()) / self.r;
        self.angular_velocity += self.angular_acceleration;
        self.angle += self.angular_velocity;

        // dampening to simulate friction / air resistence
        self.angular_velocity *= 0.99;
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        // location from polar coords
        // sin / cos swapped here because x is the y-axis and y is the x-axis
        // when looking at the graph of a pendulum
        let location =
            self.origin + DVec2::new(self.r * self.angle.sin(), self.r * self.angle.cos());

        core::stroke_grayscale(screen, 0.0);
        core::fill_grayscale(screen, 175.0);
        core::shapes::line(screen, self.origin.x, self.origin.y, location.x, location.y)?;
        core::shapes::ellipse(screen, location.x, location.y, 16.0, 16.0)?;

        Ok(())
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen, pendulum: &mut Pendulum) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    pendulum.update(0.4);
    pendulum.display(screen)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let pendulum = Rc::new(RefCell::new(None));

    core::run(
        || {
            let screen = setup()?;
            *pendulum.borrow_mut() = Some(Pendulum::new(
                DVec2::new(screen.width() as f64 / 2.0, 10.0),
                125.0,
            ));
            Ok(screen)
        },
        |screen, _| draw(screen, pendulum.borrow_mut().as_mut().unwrap()),
    )?;

    Ok(())
}
