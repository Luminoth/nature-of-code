use processing::errors::ProcessingErr;
use processing::Screen;

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen, tx: &mut f64, ty: &mut f64) -> Result<(), ProcessingErr> {
    let x = core::math::map(core::noise(*tx), 0.0, 1.0, 0.0, screen.width() as f64);
    let y = core::math::map(core::noise(*ty), 0.0, 1.0, 0.0, screen.height() as f64);
    core::shapes::ellipse(screen, x, y, 16.0, 16.0)?;

    *tx += 0.01;
    *ty += 0.01;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let mut tx = 0.0;
    let mut ty = 1000.0;

    core::run(setup, |screen| draw(screen, &mut tx, &mut ty))?;

    Ok(())
}
