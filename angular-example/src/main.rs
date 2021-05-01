use std::cell::RefCell;
use std::rc::Rc;

use glam::DVec2;
use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

#[derive(Debug, Default)]
struct Wind {
    accumulator: f64,
}

#[derive(Debug, Default)]
struct Mover {
    location: DVec2,
    velocity: DVec2,
    acceleration: DVec2,
    mass: f64,

    angle: f64,
    angular_velocity: f64,
    angular_acceleration: f64,
}

impl Mover {
    fn new(mass: f64, x: f64, y: f64) -> Self {
        Self {
            location: DVec2::new(x, y),
            mass,
            ..Default::default()
        }
    }

    fn size(&self) -> f64 {
        self.mass * 16.0
    }

    fn bounce_edges(&mut self, screen: &Screen) {
        let hs = self.size() / 2.0;

        if (self.location.x + hs) > screen.width() as f64 {
            self.location.x = screen.width() as f64 - hs;
            self.velocity.x *= -1.0;
        } else if self.location.x < hs {
            self.location.x = hs;
            self.velocity.x *= -1.0;
        }

        if (self.location.y + hs) > screen.height() as f64 {
            self.location.y = screen.height() as f64 - hs;
            self.velocity.y *= -1.0;
        } else if self.location.y < hs {
            self.location.y = hs;
            self.velocity.y *= -1.0;
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

        self.angular_velocity = core::math::clampf(
            self.angular_velocity + self.angular_acceleration * dt,
            -100.0,
            100.0,
        );
        self.angle += self.angular_velocity * dt;

        // special sauce
        self.angular_acceleration = self.acceleration.x * 10.0;

        self.acceleration = DVec2::default();
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        core::stroke_grayscale(screen, 0.0);
        core::fill_grayscale_alpha(screen, 175.0, 200.0);

        screen.push_matrix();

        core::translate(screen, self.location.x, self.location.y);
        core::rotate(screen, self.angle);

        screen.rect_mode(&core::shapes::RectMode::Center.to_string());
        core::shapes::rect(screen, 0.0, 0.0, self.size(), self.size())?;

        screen.pop_matrix();

        Ok(())
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(
    screen: &mut Screen,
    dt: f64,
    mut movers: impl AsMut<[Mover]>,
    wind: &mut Wind,
) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    wind.accumulator += dt;

    let wind_force = DVec2::new(core::noise(wind.accumulator, 0.5), 0.0) * 20.0;
    let gravity = DVec2::new(0.0, 100.0);

    for mover in movers.as_mut() {
        mover.apply_force(wind_force);
        mover.apply_force(gravity * mover.mass);

        mover.update(dt);
        mover.bounce_edges(screen);
        mover.display(screen)?;
    }

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let movers = Rc::new(RefCell::new(None));
    let wind = Rc::new(RefCell::new(None));

    core::run(
        || {
            let mut rng = rand::thread_rng();
            let screen = setup()?;

            let mut mvrs = vec![];
            for _ in 0..100 {
                let x = rng.gen_range(0..screen.width()) as f64;
                let y = rng.gen_range(0..screen.height() / 4) as f64;
                mvrs.push(Mover::new(rng.gen_range(0.1..5.0), x, y));
            }
            *movers.borrow_mut() = Some(mvrs);

            *wind.borrow_mut() = Some(Wind::default());

            Ok(screen)
        },
        |screen, dt| {
            draw(
                screen,
                dt,
                movers.borrow_mut().as_mut().unwrap(),
                wind.borrow_mut().as_mut().unwrap(),
            )
        },
    )?;

    Ok(())
}
