use glam::DVec2;
use processing::errors::ProcessingErr;
use processing::Screen;

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen, _: f64) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    let mut mouse = DVec2::new(screen.mouse_x(), screen.mouse_y());
    let center = DVec2::new(screen.width() as f64 / 2.0, screen.height() as f64 / 2.0);
    mouse -= center;

    let m = mouse.length();
    core::fill_grayscale(screen, 0.0);
    core::shapes::rect(screen, 0.0, 0.0, m, 10.0)?;

    core::translate(screen, center.x, center.y);
    core::shapes::line(screen, 0.0, 0.0, mouse.x, mouse.y)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    core::run(setup, draw)?;

    Ok(())
}
