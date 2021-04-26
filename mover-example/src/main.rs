use std::cell::RefCell;
use std::rc::Rc;

use glam::Vec2;
use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

#[derive(Debug, Default)]
struct Liquid {
    location: Vec2,
    size: Vec2,
    c: f32,
}

impl Liquid {
    #[allow(clippy::many_single_char_names)]
    fn new(x: f32, y: f32, w: f32, h: f32, c: f32) -> Self {
        Self {
            location: Vec2::new(x, y),
            size: Vec2::new(w, h),
            c,
        }
    }

    fn contains(&self, mover: &Mover) -> bool {
        mover.location.x > self.location.x
            && mover.location.x < self.location.x + self.size.x
            && mover.location.y > self.location.y
            && mover.location.y < self.location.y + self.size.y
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        screen.stroke_off();
        core::fill_grayscale(screen, 175.0);

        core::shapes::rect(
            screen,
            self.location.x as f64,
            self.location.y as f64,
            self.size.x as f64,
            self.size.y as f64,
        )
    }
}

#[derive(Debug, Default)]
struct Mover {
    location: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32,
}

impl Mover {
    fn new(mass: f32, x: f32, y: f32) -> Self {
        Self {
            location: Vec2::new(x, y),
            mass,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    fn wrap_edges(&mut self, screen: &Screen) {
        if self.location.x > screen.width() as f32 {
            self.location.x = 0.0;
        } else if self.location.x < 0.0 {
            self.location.x = screen.width() as f32;
        }

        if self.location.y > screen.height() as f32 {
            self.location.y = 0.0;
        } else if self.location.y < 0.0 {
            self.location.y = screen.height() as f32;
        }
    }

    #[allow(dead_code)]
    fn stop_edges(&mut self, screen: &Screen) {
        if self.location.x > screen.width() as f32 {
            self.location.x = screen.width() as f32;
        } else if self.location.x < 0.0 {
            self.location.x = 0.0;
        }

        if self.location.y > screen.height() as f32 {
            self.location.y = screen.height() as f32;
        } else if self.location.y < 0.0 {
            self.location.y = 0.0;
        }
    }

    #[allow(dead_code)]
    fn bounce_edges(&mut self, screen: &Screen) {
        if self.location.x > screen.width() as f32 {
            self.location.x = screen.width() as f32;
            self.velocity.x *= -1.0;
        } else if self.location.x < 0.0 {
            self.location.x = 0.0;
            self.velocity.x *= -1.0;
        }

        if self.location.y > screen.height() as f32 {
            self.location.y = screen.height() as f32;
            self.velocity.y *= -1.0;
        } else if self.location.y < 0.0 {
            self.location.y = 0.0;
            self.velocity.y *= -1.0;
        }
    }

    fn apply_force(&mut self, force: Vec2) {
        let force = force / self.mass;
        self.acceleration += force;
    }

    #[allow(dead_code)]
    fn apply_noise_force(&mut self) {
        let mut rng = rand::thread_rng();

        self.apply_force(
            core::math::vector2_random() * core::sample_noise2d() as f32 * rng.gen_range(0.1..0.5),
        );
    }

    fn drag(&mut self, liquid: &Liquid) {
        let speed = self.velocity.length();
        let drag_magnitude = liquid.c * speed * speed;

        let drag = (self.velocity * -1.0).normalize_or_zero() * drag_magnitude;
        self.apply_force(drag);
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.location += self.velocity;

        self.acceleration = Vec2::default();
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        core::stroke_grayscale(screen, 0.0);
        core::fill_grayscale(screen, 0.0);

        core::shapes::ellipse(
            screen,
            self.location.x as f64,
            self.location.y as f64,
            self.mass as f64 * 16.0,
            self.mass as f64 * 16.0,
        )
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(
    screen: &mut Screen,
    movers: &mut impl AsMut<[Mover]>,
    liquid: &Liquid,
) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    liquid.display(screen)?;

    //let wind = Vector2::new(0.01, 0.0);
    let gravity = Vec2::new(0.0, 0.1);

    //let c = 0.01;

    for mover in movers.as_mut() {
        /*let friction = (mover.velocity * -1.0).normalize_or_zero() * c;
        mover.apply_force(friction);*/

        if liquid.contains(mover) {
            mover.drag(liquid);
        }

        //mover.apply_force(wind);
        mover.apply_force(gravity * mover.mass);

        mover.update();
        mover.bounce_edges(screen);
        mover.display(screen)?;
    }

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let movers = Rc::new(RefCell::new(None));
    let liquid = Rc::new(RefCell::new(None));

    core::run(
        || {
            let mut rng = rand::thread_rng();

            let screen = setup()?;

            let mut mvrs = vec![];
            for _ in 0..100 {
                let x = rng.gen_range(0..screen.width()) as f32;
                let y = rng.gen_range(0..screen.height() / 4) as f32;
                mvrs.push(Mover::new(rng.gen_range(0.1..5.0), x, y));
            }
            *movers.borrow_mut() = Some(mvrs);

            *liquid.borrow_mut() = Some(Liquid::new(
                0.0,
                screen.height() as f32 / 2.0,
                screen.width() as f32,
                screen.height() as f32 / 2.0,
                0.1,
            ));

            Ok(screen)
        },
        |screen, _| {
            draw(
                screen,
                movers.borrow_mut().as_mut().unwrap(),
                liquid.borrow().as_ref().unwrap(),
            )
        },
    )?;

    Ok(())
}
