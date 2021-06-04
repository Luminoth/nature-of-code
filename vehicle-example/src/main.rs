use glam::DVec2;
use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

#[derive(Debug)]
struct Vehicle {
    location: DVec2,
    velocity: DVec2,
    acceleration: DVec2,
    r: f64,
    mass: f64,
    maxspeed: f64,
    maxforce: f64,
}

impl Vehicle {
    fn new(x: f64, y: f64) -> Self {
        Self {
            location: DVec2::new(x, y),
            velocity: DVec2::default(),
            acceleration: DVec2::default(),
            r: 3.0,
            mass: 1.0,
            maxspeed: 4.0,
            maxforce: 0.1,
        }
    }

    fn wander(&mut self, target: DVec2, r: f64) {
        let mut rng = rand::thread_rng();

        // target a random point on a circle around the target
        let theta = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
        let target = target + r * DVec2::new(theta.cos(), theta.sin());

        self.seek(target);
    }

    fn seek(&mut self, target: DVec2) {
        // desired velocity direction
        let desired = target - self.location;

        // desired velocity magnitude
        let d = desired.length();
        let m = if d < 100.0 {
            // slow down on arrival
            core::math::map(d, 0.0, 100.0, 0.0, self.maxspeed)
        } else {
            self.maxspeed
        };

        let desired = desired.normalize_or_zero() * m;

        // apply the steering force
        let steer = (desired - self.velocity).clamp_length_max(self.maxforce);
        self.apply_force(steer);
    }

    fn pursuit(&mut self, target: DVec2, velocity: DVec2, _dt: f64) {
        let predicted = target + velocity; // * dt;
        self.seek(predicted);
    }

    fn flee(&mut self, target: DVec2) {
        let desired = -(target - self.location).normalize_or_zero() * self.maxspeed;

        let steer = (desired - self.velocity).clamp_length_max(self.maxforce);
        self.apply_force(steer);
    }

    fn apply_force(&mut self, force: DVec2) {
        let force = if self.mass > 0.0 {
            force / self.mass
        } else {
            force
        };
        self.acceleration += force;
    }

    fn update(&mut self, _dt: f64) {
        self.velocity += self.acceleration; // * dt;
        self.velocity.clamp_length_max(self.maxspeed);

        self.location += self.velocity; // * dt;

        self.acceleration = DVec2::default();
    }

    fn heading(&self) -> f64 {
        self.velocity.x.atan2(self.velocity.y)
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        let theta = self.heading();

        core::stroke_grayscale(screen, 0.0);
        core::fill_grayscale(screen, 175.0);

        screen.push_matrix();

        core::translate(screen, self.location.x, self.location.y);
        core::rotate(screen, theta);

        // TODO: beginShape();

        //vertex(0, -self.r * 2.0);
        //vertex(-self.r, self.r * 2.0);
        //vertex(self.r, self.r * 2.0);

        // TODO: endShape(CLOSE);

        screen.pop_matrix();

        Ok(())
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen, _dt: f64) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    core::run(setup, draw)?;

    Ok(())
}
