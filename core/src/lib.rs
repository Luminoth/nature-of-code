pub mod input;
pub mod shapes;

use processing::errors::ProcessingErr;
use processing::Screen;

pub fn run<'a, S, D>(setup: S, draw: D) -> Result<(), ProcessingErr>
where
    S: FnOnce() -> Result<Screen<'a>, ProcessingErr>,
    D: Fn(&mut Screen) -> Result<(), ProcessingErr>,
{
    let mut screen = setup()?;

    loop {
        input::update(&mut screen);

        draw(&mut screen)?;

        screen.reveal()?;
    }
}

pub fn create_canvas<'a>(width: u32, height: u32) -> Result<Screen<'a>, ProcessingErr> {
    let mut screen = Screen::new(width, height, false, false, true)?;
    screen.background(1.0, 1.0, 1.0, 1.0);
    Ok(screen)
}

/* background */

pub fn background_grayscale(screen: &mut Screen, v: f32) {
    background_rgb(screen, v, v, v);
}

pub fn background_rgb(screen: &mut Screen, r: f32, g: f32, b: f32) {
    screen.background(r / 255.0, g / 255.0, b / 255.0, 1.0);
}

/* stroke */

pub fn stroke_grayscale(screen: &mut Screen, v: f32) {
    stroke_rgb(screen, v, v, v);
}

pub fn stroke_rgb(screen: &mut Screen, r: f32, g: f32, b: f32) {
    screen.fill(&[r / 255.0], &[g / 255.0], &[b / 255.0], &[1.0]);
}

/* fill */

pub fn fill_grayscale(screen: &mut Screen, v: f32) {
    fill_rgb(screen, v, v, v);
}

pub fn fill_grayscale_alpha(screen: &mut Screen, v: f32, a: f32) {
    fill_rgba(screen, v, v, v, a);
}

pub fn fill_rgb(screen: &mut Screen, r: f32, g: f32, b: f32) {
    fill_rgba(screen, r, g, b, 255.0);
}

pub fn fill_rgba(screen: &mut Screen, r: f32, g: f32, b: f32, a: f32) {
    screen.fill(&[r / 255.0], &[g / 255.0], &[b / 255.0], &[a / 255.0]);
}

/* internal utils */

pub(crate) fn x_to_screen(screen: &Screen, v: f64) -> f64 {
    let hw = screen.width() as f64 / 2.0;
    (v - hw) / hw
}

pub(crate) fn y_to_screen(screen: &Screen, v: f64) -> f64 {
    let hh = screen.height() as f64 / 2.0;
    (hh - v) / hh
}

pub(crate) fn w_to_screen(screen: &Screen, v: f64) -> f64 {
    let hw = screen.width() as f64 / 2.0;
    v / hw
}

pub(crate) fn h_to_screen(screen: &Screen, v: f64) -> f64 {
    let hh = screen.height() as f64 / 2.0;
    v / hh
}
