use std::cell::RefCell;
use std::rc::Rc;

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

    #[allow(dead_code)]
    fn follow_flow(&mut self, flow: &FlowField) {
        // desired velocity from the flow field
        let desired = flow.lookup(self.location) * self.maxspeed;

        // apply the steering force
        let steer = (desired - self.velocity).clamp_length_max(self.maxforce);
        self.apply_force(steer);
    }

    #[allow(dead_code)]
    fn follow_path(&mut self, path: &Path, _dt: f64) {
        // predict 25 pixels ahead
        let predict = self.location + self.velocity.normalize_or_zero() * 25.0; // * dt;

        let mut target = None;
        let mut world_record = 1000000.0;

        // find the normal on the closest path segment
        for i in 0..path.points.len() - 1 {
            let a = path.points[i];
            let b = path.points[i + 1];

            // project the predicted point onto the segment
            let mut proj = core::math::project(predict, a, b);

            // if we're not on this segment, hackily take the "end" as the normal
            if proj.x < a.x.min(b.x) || proj.x > a.x.max(b.x) {
                proj = b;
            }

            let distance = predict.distance(proj);
            if distance < world_record {
                world_record = distance;

                // target 10 pixels out from the predicted point
                let dir = (b - a).normalize_or_zero();
                target = Some(proj + dir * 10.0);
            }
        }

        // only seek the target if we're outside the path radius
        if let Some(target) = target {
            if world_record > path.radius {
                self.seek(target);
            }
        }
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn pursuit(&mut self, target: DVec2, velocity: DVec2, _dt: f64) {
        let predicted = target + velocity; // * dt;
        self.seek(predicted);
    }

    #[allow(dead_code)]
    fn flee(&mut self, target: DVec2) {
        let desired = -(target - self.location).normalize_or_zero() * self.maxspeed;

        // apply the steering force
        let steer = (desired - self.velocity).clamp_length_max(self.maxforce);
        self.apply_force(steer);
    }

    fn separate(&mut self, vehicles: impl AsRef<[RefCell<Vehicle>]>) {
        let desired_separation = self.r * 10.0;

        let mut sum = DVec2::default();
        let mut count = 0;

        for other in vehicles.as_ref().iter() {
            // this check should stop us comparing against ourself
            if let Ok(other) = other.try_borrow() {
                let d = self.location.distance(other.location);
                if d < desired_separation {
                    let mut diff = (self.location - other.location).normalize_or_zero();

                    // weight how fast we flee by the distance
                    diff /= d;

                    sum += diff;
                    count += 1;
                }
            }
        }

        if count > 0 {
            sum /= count as f64;

            // set the magnitude to the max speed
            sum *= self.maxspeed / sum.length();

            // apply the steering force
            let steer = (sum - self.velocity).clamp_length_max(self.maxforce);
            self.apply_force(steer);
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

        /*
        // TODO: beginShape();

        //vertex(0, -self.r * 2.0);
        //vertex(-self.r, self.r * 2.0);
        //vertex(self.r, self.r * 2.0);

        // TODO: endShape();
        */

        core::shapes::ellipse(
            screen,
            self.location.x,
            self.location.y,
            self.r * 2.0,
            self.r * 2.0,
        )?;

        screen.pop_matrix();

        Ok(())
    }
}

struct FlowField {
    field: Vec<Vec<DVec2>>,
    cols: usize,
    rows: usize,
    resolution: usize,
}

impl FlowField {
    #[allow(dead_code)]
    fn noise_field(cols: usize, rows: usize) -> Vec<Vec<DVec2>> {
        let mut field = Vec::with_capacity(cols);

        let mut xoff = 0.0;
        for _ in 0..cols {
            let mut row = Vec::with_capacity(rows);

            let mut yoff = 0.0;
            for _ in 0..rows {
                let theta = core::math::map(
                    core::noise2d([xoff, yoff], 0.5),
                    0.0,
                    1.0,
                    0.0,
                    2.0 * std::f64::consts::PI,
                );
                row.push(DVec2::new(theta.cos(), theta.sin()));

                yoff += 0.1;
            }
            field.push(row);

            xoff += 0.1;
        }

        field
    }

    #[allow(dead_code)]
    fn new(screen: &Screen, resolution: usize) -> Self {
        let cols = screen.width() as usize / resolution;
        let rows = screen.height() as usize / resolution;

        Self {
            field: FlowField::noise_field(cols, rows),
            cols,
            rows,
            resolution,
        }
    }

    fn lookup(&self, pos: DVec2) -> DVec2 {
        let col = core::math::clamp(pos.x as usize / self.resolution, 0, self.cols - 1);
        let row = core::math::clamp(pos.y as usize / self.resolution, 0, self.rows - 1);

        self.field[col][row]
    }
}

struct Path {
    points: Vec<DVec2>,
    radius: f64,
}

impl Path {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            points: vec![],
            radius: 20.0,
        }
    }

    #[allow(dead_code)]
    fn add_point(&mut self, x: f64, y: f64) {
        self.points.push(DVec2::new(x, y));
    }

    #[allow(dead_code)]
    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        core::stroke_grayscale(screen, 0.0);
        screen.fill_off();

        // TODO: beginShape();

        for _v in self.points.iter() {
            //vertex(v.x, v.y);
        }

        // TODO: endShape(CLOSE);

        Ok(())
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(
    screen: &mut Screen,
    dt: f64,
    vehicles: impl AsRef<[RefCell<Vehicle>]>,
) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    for v in vehicles.as_ref().iter() {
        let mut v = v.borrow_mut();

        v.separate(&vehicles);

        v.update(dt);
        v.display(screen)?;
    }

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let vehicles = Rc::new(RefCell::new(None));

    core::run(
        || {
            let mut rng = rand::thread_rng();

            let screen = setup()?;

            let mut vs = vec![];
            for _ in 0..100 {
                vs.push(RefCell::new(Vehicle::new(
                    rng.gen_range(0..screen.width()) as f64,
                    rng.gen_range(0..screen.height()) as f64,
                )));
            }

            *vehicles.borrow_mut() = Some(vs);

            Ok(screen)
        },
        |screen, dt| draw(screen, dt, vehicles.borrow_mut().as_mut().unwrap()),
    )?;

    Ok(())
}
