pub mod input;
pub mod shapes;

use noise::{NoiseFn, Perlin, Seedable};
use once_cell::sync::Lazy;
use processing::errors::ProcessingErr;
use processing::Screen;
use rand::random;

pub fn run<'a, S, D>(setup: S, mut draw: D) -> Result<(), ProcessingErr>
where
    S: FnOnce() -> Result<Screen<'a>, ProcessingErr>,
    D: FnMut(&mut Screen) -> Result<(), ProcessingErr>,
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

/* utils */

static PERLIN: Lazy<Perlin> = Lazy::new(|| Perlin::new().set_seed(random()));

pub fn noise(point: f64) -> f64 {
    map(PERLIN.get([point, 0.0]), -1.0, 1.0, 0.0, 1.0)
}

pub fn noise2d(point: [f64; 2]) -> f64 {
    map(PERLIN.get(point), -1.0, 1.0, 0.0, 1.0)
}

pub fn noise3d(point: [f64; 2]) -> f64 {
    map(PERLIN.get(point), -1.0, 1.0, 0.0, 1.0)
}

// https://www.arduino.cc/reference/en/language/functions/math/map/
pub fn map(x: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
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
