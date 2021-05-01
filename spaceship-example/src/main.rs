use std::cell::RefCell;
use std::rc::Rc;

use glam::DVec2;
use processing::errors::ProcessingErr;
use processing::Screen;

#[derive(Debug, Default)]
struct Spaceship {
    location: DVec2,
    velocity: DVec2,
    acceleration: DVec2,
    mass: f64,

    angle: f64,
}

impl Spaceship {
    fn new(mass: f64, x: f64, y: f64) -> Self {
        Self {
            location: DVec2::new(x, y),
            mass,
            ..Default::default()
        }
    }

    fn heading(&self) -> DVec2 {
        let x = self.angle.cos();
        let y = self.angle.sin();
        DVec2::new(x, y).normalize()
    }

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

    fn apply_force(&mut self, force: DVec2) {
        let force = if self.mass > 0.0 {
            force / self.mass
        } else {
            force
        };
        self.acceleration += force;
    }

    fn update(&mut self, dt: f64) {
        self.velocity += self.acceleration * dt;
        self.location += self.velocity * dt;

        self.acceleration = DVec2::default();
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        core::stroke_grayscale(screen, 0.0);
        core::fill_grayscale(screen, 175.0);

        screen.push_matrix();

        core::translate(screen, self.location.x, self.location.y);
        core::rotate(screen, self.angle);

        // thrusters
        screen.rect_mode(&core::shapes::RectMode::Center.to_string());
        core::shapes::rect(screen, -8.0, 8.0, 8.0, 8.0)?;
        core::shapes::rect(screen, -8.0, -8.0, 8.0, 8.0)?;

        // main ship
        core::shapes::triangle(screen, -16.0, 16.0, -16.0, -16.0, 16.0, 0.0)?;

        screen.pop_matrix();

        Ok(())
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen, dt: f64, spaceship: &mut Spaceship) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    if screen.key_press(processing::Key::Left) {
        spaceship.angle += 0.1;
    }
    if screen.key_press(processing::Key::Right) {
        spaceship.angle -= 0.1;
    }

    if screen.key_press(processing::Key::Z) {
        spaceship.apply_force(spaceship.heading() * 500.0);
    }

    spaceship.update(dt);
    spaceship.stop_edges(screen);
    spaceship.display(screen)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let spaceship = Rc::new(RefCell::new(None));

    core::run(
        || {
            let screen = setup()?;
            *spaceship.borrow_mut() = Some(Spaceship::new(
                10.0,
                screen.width() as f64 / 2.0,
                screen.height() as f64 / 2.0,
            ));
            Ok(screen)
        },
        |screen, dt| draw(screen, dt, spaceship.borrow_mut().as_mut().unwrap()),
    )?;

    Ok(())
}
