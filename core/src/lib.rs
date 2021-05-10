pub mod input;
pub mod math;
pub mod shapes;

use std::time::Instant;

use noise::{NoiseFn, Perlin, Seedable};
use once_cell::sync::Lazy;
use processing::errors::ProcessingErr;
use processing::Screen;
use rand::random;

use crate::math::*;

pub fn run<'a, S, D>(setup: S, mut draw: D) -> Result<(), ProcessingErr>
where
    S: FnOnce() -> Result<Screen<'a>, ProcessingErr>,
    D: FnMut(&mut Screen, f64) -> Result<(), ProcessingErr>,
{
    let mut screen = setup()?;

    let mut prev = Instant::now();
    loop {
        input::update(&mut screen);

        screen.reset_matrix();

        let now = Instant::now();
        draw(&mut screen, (Instant::now() - prev).as_secs_f64())?;
        prev = now;

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
        1.0 + x_to_screen(screen, x) as f32,
        -1.0 + y_to_screen(screen, y) as f32,
        0.0,
    );
}

pub fn rotate(screen: &mut Screen, angle: f64) {
    screen.rotate_z(angle as f32);
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

pub fn stroke_grayscale_alpha(screen: &mut Screen, v: f32, a: f32) {
    stroke_rgba(screen, v, v, v, a);
}

pub fn stroke_rgb(screen: &mut Screen, r: f32, g: f32, b: f32) {
    stroke_rgba(screen, r, g, b, 255.0);
}

pub fn stroke_rgba(screen: &mut Screen, r: f32, g: f32, b: f32, a: f32) {
    screen.fill(&[r / 255.0], &[g / 255.0], &[b / 255.0], &[a / 255.0]);
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

/* image */

pub fn image(
    screen: &mut Screen,
    x: f64,
    y: f64,
    texture: &processing::Texture2d,
) -> Result<(), ProcessingErr> {
    let mut rect = processing::shapes::rect::Rect::new(
        screen,
        &[x_to_screen(screen, x)],
        &[y_to_screen(screen, y)],
        &[0.0],
        &[w_to_screen(screen, texture.width() as f64)],
        &[h_to_screen(screen, texture.height() as f64)],
    )?;
    rect.attach_texture(texture);
    screen.draw(&rect)
}

/* noise */

#[allow(dead_code)]
static PERLIN_NOISE: Lazy<Perlin> = Lazy::new(|| Perlin::default().set_seed(random()));

/// Output range [-1..1]
pub fn noise(point: f64, frequency: f64) -> f64 {
    let point = [point * frequency, 0.0];
    PERLIN_NOISE.get(point)
}

/// Output range [-1..1]
pub fn noise2d(point: [f64; 2], frequency: f64) -> f64 {
    let point = [point[0] * frequency, point[1] * frequency];
    PERLIN_NOISE.get(point)
}

/// Output range [-1..1]
pub fn noise3d(point: [f64; 3], frequency: f64) -> f64 {
    let point = [
        point[0] * frequency,
        point[1] * frequency,
        point[2] * frequency,
    ];
    PERLIN_NOISE.get(point)
}

/* internal utils */

pub(crate) fn x_to_screen(screen: &Screen, x: f64) -> f64 {
    map(x, 0.0, screen.width() as f64, -1.0, 1.0)
}

pub(crate) fn y_to_screen(screen: &Screen, y: f64) -> f64 {
    -map(y, 0.0, screen.height() as f64, -1.0, 1.0)
}

pub(crate) fn w_to_screen(screen: &Screen, w: f64) -> f64 {
    let hw = screen.width() as f64 / 2.0;
    w / hw
}

pub(crate) fn h_to_screen(screen: &Screen, h: f64) -> f64 {
    let hh = screen.height() as f64 / 2.0;
    h / hh
}
