use processing::errors::ProcessingErr;
use processing::Screen;

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    let mut screen = core::create_canvas(400, 400)?;
    core::background_grayscale(&mut screen, 220.0);
    Ok(screen)
}

fn draw(screen: &mut Screen, r: f64, theta: &mut f64) -> Result<(), ProcessingErr> {
    let x = r * theta.cos();
    let y = r * theta.sin();

    screen.stroke_off();
    core::fill_grayscale(screen, 0.0);

    core::shapes::ellipse(
        screen,
        x + screen.width() as f64 / 2.0,
        y + screen.height() as f64 / 2.0,
        16.0,
        16.0,
    )?;

    *theta += 0.01;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let r = 75.0;
    let mut theta = 0.0;

    core::run(setup, |screen, _| draw(screen, r, &mut theta))?;

    Ok(())
}
