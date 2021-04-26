use glam::Vec2;
use processing::errors::ProcessingErr;
use processing::Screen;

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen, _: f64) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    let mut mouse = Vec2::new(screen.mouse_x() as f32, screen.mouse_y() as f32);
    let center = Vec2::new(screen.width() as f32 / 2.0, screen.height() as f32 / 2.0);
    mouse -= center;

    let m = mouse.length();
    core::fill_grayscale(screen, 0.0);
    core::shapes::rect(screen, 0.0, 0.0, m as f64, 10.0)?;

    core::translate(screen, center.x as f64, center.y as f64);
    core::shapes::line(screen, 0.0, 0.0, mouse.x as f64, mouse.y as f64)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    core::run(setup, draw)?;

    Ok(())
}
