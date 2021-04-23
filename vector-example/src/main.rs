use nalgebra::Vector2;
use processing::errors::ProcessingErr;
use processing::Screen;

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    let mut mouse = Vector2::new(screen.mouse_x(), screen.mouse_y());
    let center = Vector2::new(screen.width() as f64 / 2.0, screen.height() as f64 / 2.0);
    mouse -= center;

    let m = mouse.magnitude();
    core::fill_grayscale(screen, 0.0);
    core::shapes::rect(screen, 0.0, 0.0, m, 10.0)?;

    // TODO: this isn't quite working right
    // something in the point to pixel conversion in core is wrong
    core::translate(screen, center.x, center.y);
    core::shapes::line(screen, 0.0, 0.0, mouse.x, mouse.y)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    core::run(setup, draw)?;

    Ok(())
}
