use std::fmt;

use processing::errors::ProcessingErr;
use processing::shapes::ellipse::Ellipse;
use processing::shapes::line::Line;
use processing::shapes::point::Point;
use processing::shapes::rect::Rect;
use processing::shapes::triangle::Triangle;
use processing::Screen;

use crate::{device_to_screen, device_to_screen_size};

pub enum RectMode {
    Center,
    Radius,
    Corner,
}

impl fmt::Display for RectMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Center => write!(f, "CENTER"),
            Self::Radius => write!(f, "RADIUS"),
            Self::Corner => write!(f, "CORNERS"),
        }
    }
}

pub fn point(screen: &mut Screen, x: f64, y: f64) -> Result<(), ProcessingErr> {
    let (x, y) = device_to_screen(screen, x, y);

    let point = Point::new(screen, &[x], &[y], &[0.0])?;
    screen.draw(&point)
}

pub fn line(screen: &mut Screen, x1: f64, y1: f64, x2: f64, y2: f64) -> Result<(), ProcessingErr> {
    let (x1, y1) = device_to_screen(screen, x1, y1);
    let (x2, y2) = device_to_screen(screen, x2, y2);

    let line = Line::new(screen, &[x1], &[y1], &[0.0], &[x2], &[y2], &[0.0])?;
    screen.draw(&line)
}

pub fn triangle(
    screen: &mut Screen,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
) -> Result<(), ProcessingErr> {
    let (x1, y1) = device_to_screen(screen, x1, y1);
    let (x2, y2) = device_to_screen(screen, x2, y2);
    let (x3, y3) = device_to_screen(screen, x3, y3);

    let triangle = Triangle::new(
        screen,
        &[x1],
        &[y1],
        &[0.0],
        &[x2],
        &[y2],
        &[0.0],
        &[x3],
        &[y3],
        &[0.0],
    )?;
    screen.draw(&triangle)
}

pub fn rect(screen: &mut Screen, x: f64, y: f64, w: f64, h: f64) -> Result<(), ProcessingErr> {
    let (x, y) = device_to_screen(screen, x, y);
    let (w, h) = device_to_screen_size(screen, w, h);

    let rect = Rect::new(screen, &[x], &[y], &[0.0], &[w], &[h])?;
    screen.draw(&rect)
}

pub fn ellipse(screen: &mut Screen, x: f64, y: f64, w: f64, h: f64) -> Result<(), ProcessingErr> {
    let (x, y) = device_to_screen(screen, x, y);
    let (w, h) = device_to_screen_size(screen, w, h);

    let ellipse = Ellipse::new(screen, &[x], &[y], &[0.0], &[w], &[h])?;
    screen.draw(&ellipse)
}
