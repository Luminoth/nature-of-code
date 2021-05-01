use processing::errors::ProcessingErr;
use processing::Screen;

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(
    screen: &mut Screen,
    angle: &mut f64,
    avelocity: &mut f64,
    aacceleration: &mut f64,
) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    core::fill_grayscale(screen, 175.0);
    core::stroke_grayscale(screen, 0.0);
    screen.rect_mode(&core::shapes::RectMode::Center.to_string());

    core::translate(
        screen,
        screen.width() as f64 / 2.0,
        screen.height() as f64 / 2.0,
    );
    core::rotate(screen, *angle);

    core::shapes::line(screen, -50.0, 0.0, 50.0, 0.0)?;
    core::shapes::ellipse(screen, 50.0, 0.0, 8.0, 8.0)?;
    core::shapes::ellipse(screen, -50.0, 0.0, 8.0, 8.0)?;

    *avelocity += *aacceleration;
    *angle += *avelocity;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let mut angle = 0.0;
    let mut avelocity = 0.0;
    let mut aacceleration = 0.001;

    core::run(setup, |screen, _| {
        draw(screen, &mut angle, &mut avelocity, &mut aacceleration)
    })?;

    Ok(())
}
