pub mod input;
pub mod math;
pub mod shapes;

use noise::{NoiseFn, Perlin, Seedable};
use once_cell::sync::Lazy;
use processing::errors::ProcessingErr;
use processing::Screen;
use rand::random;

use crate::math::*;

pub fn run<'a, S, D>(setup: S, mut draw: D) -> Result<(), ProcessingErr>
where
    S: FnOnce() -> Result<Screen<'a>, ProcessingErr>,
    D: FnMut(&mut Screen) -> Result<(), ProcessingErr>,
{
    let mut screen = setup()?;

    loop {
        input::update(&mut screen);

        screen.reset_matrix();
        draw(&mut screen)?;

        screen.reveal()?;
    }
}

pub fn create_canvas<'a>(width: u32, height: u32) -> Result<Screen<'a>, ProcessingErr> {
    let mut screen = Screen::new(width, height, false, false, true)?;
    screen.background(1.0, 1.0, 1.0, 1.0);
    Ok(screen)
}

pub fn translate(screen: &mut Screen, x: f64, y: f64) {
    screen.translate(
        x_to_screen(screen, x) as f32,
        y_to_screen(screen, y) as f32,
        0.0,
    );
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

pub fn noise3d(point: [f64; 3]) -> f64 {
    map(PERLIN.get(point), -1.0, 1.0, 0.0, 1.0)
}

/* internal utils */

pub(crate) fn x_to_screen(screen: &Screen, x: f64) -> f64 {
    let hw = screen.width() as f64 / 2.0;
    (x - hw) / hw
}

pub(crate) fn y_to_screen(screen: &Screen, y: f64) -> f64 {
    let hh = screen.height() as f64 / 2.0;
    (hh - y) / hh
}

pub(crate) fn w_to_screen(screen: &Screen, w: f64) -> f64 {
    let hw = screen.width() as f64 / 2.0;
    w / hw
}

pub(crate) fn h_to_screen(screen: &Screen, h: f64) -> f64 {
    let hh = screen.height() as f64 / 2.0;
    h / hh
}
