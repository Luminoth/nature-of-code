use processing::errors::ProcessingErr;
use processing::Screen;
use rand_distr::{Distribution, Normal};

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen, _: f64) -> Result<(), ProcessingErr> {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(320.0, 60.0).unwrap();

    let x = normal.sample(&mut rng);

    screen.stroke_off();
    core::fill_grayscale_alpha(screen, 0.0, 10.0);
    core::shapes::ellipse(screen, x, 180.0, 16.0, 16.0)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    core::run(setup, draw)?;

    Ok(())
}
