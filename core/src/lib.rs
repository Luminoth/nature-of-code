pub mod input;
pub mod math;
pub mod shapes;

use std::fmt;
use std::time::Instant;

use noise::{NoiseFn, Perlin, Seedable};
use once_cell::sync::Lazy;
use processing::errors::ProcessingErr;
use processing::Screen;
use rand::random;
use wrapped2d::b2;
use wrapped2d::user_data::UserDataTypes;

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

pub fn b2d_run<'a, S, D, U>(setup: S, mut draw: D) -> Result<(), ProcessingErr>
where
    S: FnOnce() -> Result<(Screen<'a>, b2::World<U>), ProcessingErr>,
    D: FnMut(&mut Screen, &mut b2::World<U>, f64) -> Result<(), ProcessingErr>,
    U: UserDataTypes,
{
    let (mut screen, mut world) = setup()?;

    let mut prev = Instant::now();
    loop {
        input::update(&mut screen);

        {
            let time_step = 1.0 / 60.0;
            world.step(time_step, 10, 8);
            world.clear_forces();
        }

        screen.reset_matrix();

        let now = Instant::now();
        draw(
            &mut screen,
            &mut world,
            (Instant::now() - prev).as_secs_f64(),
        )?;
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
    let (x, y) = device_to_screen(screen, x, y);

    screen.translate(1.0 + x as f32, -1.0 + y as f32, 0.0);
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
    let (x, y) = device_to_screen(screen, x, y);
    let (w, h) = device_to_screen_size(screen, texture.width() as f64, texture.height() as f64);

    let mut rect = processing::shapes::rect::Rect::new(screen, &[x], &[y], &[0.0], &[w], &[h])?;
    rect.attach_texture(texture);
    screen.draw(&rect)
}

/* blend mode */

pub enum BlendMode {
    Replace,
    Blend,
    Add,
    Subtract,
    Lightest,
    Darkest,
    Exclusion,
    Multiply,
    Screen,
}

impl fmt::Display for BlendMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Replace => write!(f, "REPLACE"),
            Self::Blend => write!(f, "BLEND"),
            Self::Add => write!(f, "ADD"),
            Self::Subtract => write!(f, "SUBTRACT"),
            Self::Lightest => write!(f, "LIGHTEST"),
            Self::Darkest => write!(f, "DARKEST"),
            Self::Exclusion => write!(f, "EXCLUSION"),
            Self::Multiply => write!(f, "MULTIPLY"),
            Self::Screen => write!(f, "SCREEN"),
        }
    }
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

pub(crate) fn device_to_screen(screen: &Screen, x: f64, y: f64) -> (f64, f64) {
    (
        map(x, 0.0, screen.width() as f64, -1.0, 1.0),
        -map(y, 0.0, screen.height() as f64, -1.0, 1.0),
    )
}

pub(crate) fn device_to_screen_size(screen: &Screen, w: f64, h: f64) -> (f64, f64) {
    (
        w / (screen.width() as f64 / 2.0),
        h / (screen.height() as f64 / 2.0),
    )
}

/* Box2D utils */

// https://github.com/shiffman/Box2D-for-Processing/blob/master/Box2D-for-Processing/src/shiffman/box2d/Box2DProcessing.java

const SCALE_FACTOR: f64 = 10.0;
const Y_FLIP: bool = true;

pub fn coord_world_to_pixels(screen: &Screen, wx: f64, wy: f64) -> b2::Vec2 {
    let hw = screen.width() as f64 / 2.0;
    let hh = screen.height() as f64 / 2.0;

    let px = map(wx, 0.0, 1.0, hw, hw + SCALE_FACTOR);

    let mut py = map(wy, 0.0, 1.0, hh, hh + SCALE_FACTOR);
    if Y_FLIP {
        py = map(py, 0.0, screen.height() as f64, screen.height() as f64, 0.0);
    }

    b2::Vec2 {
        x: px as f32,
        y: py as f32,
    }
}

pub fn vector_world_to_pixels(screen: &Screen, v: b2::Vec2) -> b2::Vec2 {
    coord_world_to_pixels(screen, v.x as f64, v.y as f64)
}

pub fn scalar_world_to_pixels(val: f64) -> f64 {
    val * SCALE_FACTOR
}

pub fn get_body_pixel_coord(screen: &Screen, b: &b2::Body) -> b2::Vec2 {
    let xf = b.transform();
    coord_world_to_pixels(screen, xf.pos.x as f64, xf.pos.y as f64)
}

pub fn coord_pixels_to_world(screen: &Screen, px: f64, py: f64) -> b2::Vec2 {
    let hw = screen.width() as f64 / 2.0;
    let hh = screen.height() as f64 / 2.0;

    let wx = map(px, hw, hw + SCALE_FACTOR, 0.0, 1.0);

    let mut wy = py;
    if Y_FLIP {
        wy = map(py, screen.height() as f64, 0.0, 0.0, screen.height() as f64);
    }

    wy = map(wy, hh, hh + SCALE_FACTOR, 0.0, 1.0);

    b2::Vec2 {
        x: wx as f32,
        y: wy as f32,
    }
}

pub fn vector_pixels_to_world(screen: &Screen, v: b2::Vec2) -> b2::Vec2 {
    coord_pixels_to_world(screen, v.x as f64, v.y as f64)
}

pub fn scalar_pixels_to_world(val: f64) -> f64 {
    val / SCALE_FACTOR
}
