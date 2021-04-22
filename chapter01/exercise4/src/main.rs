use processing::errors::ProcessingErr;
use processing::Screen;
use rand_distr::{Distribution, Normal};

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen, dots: impl AsRef<[(f64, f64)]>) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    screen.stroke_off();
    core::fill_rgba(screen, 255.0, 0.0, 0.0, 50.0);
    for dot in dots.as_ref() {
        core::shapes::ellipse(screen, dot.0, dot.1, 16.0, 16.0)?;
    }

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let mut dots = vec![(0.0, 0.0); 1000];

    let mut rng = rand::thread_rng();
    let normal_x = Normal::new(320.0, 60.0).unwrap();
    let normal_y = Normal::new(180.0, 30.0).unwrap();

    for i in 0..dots.len() {
        dots[i] = (normal_x.sample(&mut rng), normal_y.sample(&mut rng));
    }

    core::run(setup, |screen| draw(screen, &dots))?;

    Ok(())
}
