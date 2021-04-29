use std::cell::RefCell;
use std::rc::Rc;

use glam::DVec2;
use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

#[derive(Debug, Default)]
struct Balloon {
    location: DVec2,
    velocity: DVec2,
    acceleration: DVec2,
}

impl Balloon {
    fn new(x: f64, y: f64) -> Self {
        Self {
            location: DVec2::new(x, y),
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
            self.velocity.y *= -0.2;
        }
    }

    fn apply_force(&mut self, force: DVec2) {
        self.acceleration += force;
    }

    fn update(&mut self, dt: f64) {
        self.velocity += self.acceleration * dt;
        self.location += self.velocity * dt;

        self.acceleration = DVec2::default();
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        core::stroke_grayscale(screen, 0.0);
        core::fill_grayscale(screen, 0.0);

        core::shapes::ellipse(screen, self.location.x, self.location.y, 16.0, 16.0)
    }
}

#[derive(Debug, Default)]
struct Wind {
    accumulator: f64,
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(
    screen: &mut Screen,
    dt: f64,
    mut balloons: impl AsMut<[Balloon]>,
    wind: &mut Wind,
) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    wind.accumulator += dt;

    let balloon_force = DVec2::new(0.0, -10.0);
    let wind_force = DVec2::new(core::noise(wind.accumulator, 0.5), 0.0) * 20.0;

    for balloon in balloons.as_mut() {
        balloon.apply_force(balloon_force);
        balloon.apply_force(wind_force);

        balloon.update(dt);
        balloon.check_edges(screen);
        balloon.display(screen)?;
    }

    // stole this from dave - render the wind strength
    core::shapes::rect(screen, 300.0, 20.0, wind_force.x * 16.0, 32.0)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let balloons = Rc::new(RefCell::new(None));
    let wind = Rc::new(RefCell::new(None));

    core::run(
        || {
            let mut rng = rand::thread_rng();

            let screen = setup()?;

            let mut bs = vec![];
            for _ in 0..20 {
                let x = rng.gen_range(0..screen.width()) as f64;
                bs.push(Balloon::new(
                    x,
                    screen.height() as f64 - rng.gen_range(0.0..10.0),
                ));
            }
            *balloons.borrow_mut() = Some(bs);

            *wind.borrow_mut() = Some(Wind::default());

            Ok(screen)
        },
        |screen, dt| {
            draw(
                screen,
                dt,
                balloons.borrow_mut().as_mut().unwrap(),
                wind.borrow_mut().as_mut().unwrap(),
            )
        },
    )?;

    Ok(())
}
