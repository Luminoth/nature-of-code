use once_cell::sync::OnceCell;
use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

#[derive(Debug)]
struct Walker {
    x: i32,
    y: i32,
}
static mut WALKER: OnceCell<Walker> = OnceCell::new();

impl Walker {
    fn instance() -> &'static mut Walker {
        unsafe { WALKER.get_mut().unwrap() }
    }

    fn new(screen: &mut Screen) -> Self {
        Self {
            x: (screen.width() / 2) as i32,
            y: (screen.height() / 2) as i32,
        }
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        core::stroke_grayscale(screen, 0.0);
        core::shapes::point(screen, self.x as f64, self.y as f64)
    }

    fn step(&mut self) {
        let mut rng = rand::thread_rng();

        let stepx = rng.gen_range(0..3) - 1;
        let stepy = rng.gen_range(0..3) - 1;

        self.x += stepx;
        self.y += stepy;
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    let mut screen = core::create_canvas(640, 360)?;
    core::background_grayscale(&mut screen, 255.0);

    unsafe {
        WALKER.set(Walker::new(&mut screen)).unwrap();
    }

    Ok(screen)
}

fn draw(screen: &mut Screen) -> Result<(), ProcessingErr> {
    let walker = Walker::instance();
    walker.step();
    walker.display(screen)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    core::run(setup, draw)?;

    Ok(())
}
