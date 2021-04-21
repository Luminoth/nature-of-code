use processing::errors::ProcessingErr;
use processing::Screen;

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(400, 400)
}

fn draw(screen: &mut Screen) -> Result<(), ProcessingErr> {
    //core::background_grayscale(screen, 220.0);

    if core::input::mouse_is_pressed() {
        core::fill_grayscale(screen, 0.0)
    } else {
        core::fill_grayscale(screen, 255.0)
    }

    //core::ellipse(screen, 50.0, 50.0, 80.0, 80.0)?;

    let x = screen.mouse_x();
    let y = screen.mouse_y();
    core::shapes::ellipse(screen, x, y, 80.0, 80.0)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    core::run(setup, draw)?;

    Ok(())
}
