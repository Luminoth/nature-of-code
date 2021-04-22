use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;
use rand_distr::{Distribution, Normal};

// TODO: this is not right :(

#[derive(Debug, Default, Clone)]
struct Color {
    x: f64,
    y: f64,

    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    fn randomize(&mut self, r: &mut impl Rng, d: &impl Distribution<f32>) {
        // TODO: x / y ?

        self.r = d.sample(r);
        self.g = d.sample(r);
        self.b = d.sample(r);
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen, dots: impl AsRef<[Color]>) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    screen.stroke_off();
    for dot in dots.as_ref() {
        core::fill_rgba(screen, dot.r, dot.g, dot.b, 50.0);
        core::shapes::ellipse(screen, dot.x, dot.y, 16.0, 16.0)?;
    }

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let mut dots = vec![Color::default(); 1000];

    let mut rng = rand::thread_rng();
    let normal = Normal::new(128.0, 65.0).unwrap();

    for i in 0..dots.len() {
        dots[i].randomize(&mut rng, &normal);
    }

    core::run(setup, |screen| draw(screen, &dots))?;

    Ok(())
}
