use std::cell::RefCell;
use std::rc::Rc;

use glam::DVec2;
use processing::errors::ProcessingErr;
use processing::Screen;

#[derive(Debug)]
struct KochLine {
    start: DVec2,
    end: DVec2,
}

impl KochLine {
    fn new(a: DVec2, b: DVec2) -> Self {
        Self { start: a, end: b }
    }

    fn a(&self) -> DVec2 {
        self.start
    }

    fn b(&self) -> DVec2 {
        self.start + ((self.end - self.start) * (1.0 / 3.0))
    }

    fn c(&self) -> DVec2 {
        let mut v = (self.end - self.start) * (1.0 / 3.0);
        let a = self.start + v;

        // rotate v (copied from PVector source)
        let theta = -60.0_f64.to_radians();
        let temp = v.x;
        v.x = v.x * theta.cos() - v.y * theta.sin();
        v.y = temp * theta.sin() + v.y * theta.cos();

        a + v
    }

    fn d(&self) -> DVec2 {
        self.start + ((self.end - self.start) * (2.0 / 3.0))
    }

    fn e(&self) -> DVec2 {
        self.end
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        core::stroke_grayscale(screen, 0.0);

        core::shapes::linev(screen, self.start, self.end)?;

        Ok(())
    }
}

#[derive(Debug, Default)]
struct KochLines {
    lines: Vec<KochLine>,
}

impl KochLines {
    fn generate(&mut self) {
        let mut lines = vec![];
        for line in self.lines.iter() {
            let a = line.a();
            let b = line.b();
            let c = line.c();
            let d = line.d();
            let e = line.e();

            lines.push(KochLine::new(a, b));
            lines.push(KochLine::new(b, c));
            lines.push(KochLine::new(c, d));
            lines.push(KochLine::new(d, e));
        }
        self.lines = lines;
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        for line in self.lines.iter() {
            line.display(screen)?;
        }

        Ok(())
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen, _: f64, lines: &mut KochLines) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    lines.display(screen)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let lines = Rc::new(RefCell::new(None));

    core::run(
        || {
            let screen = setup()?;

            let mut l = vec![];

            let start = DVec2::new(0.0, 200.0);
            let end = DVec2::new(screen.width() as f64, 200.0);

            l.push(KochLine::new(start, end));

            let mut k = KochLines { lines: l };
            k.generate();
            k.generate();
            k.generate();
            k.generate();
            k.generate();

            *lines.borrow_mut() = Some(k);

            Ok(screen)
        },
        |screen, dt| draw(screen, dt, lines.borrow_mut().as_mut().unwrap()),
    )?;

    Ok(())
}
