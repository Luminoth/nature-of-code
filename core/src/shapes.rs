use processing::errors::ProcessingErr;
use processing::shapes::ellipse::Ellipse;
use processing::shapes::line::Line;
use processing::shapes::point::Point;
use processing::shapes::rect::Rect;
use processing::Screen;

use crate::{h_to_screen, w_to_screen, x_to_screen, y_to_screen};

pub fn line(screen: &mut Screen, x1: f64, y1: f64, x2: f64, y2: f64) -> Result<(), ProcessingErr> {
    let line = Line::new(
        screen,
        &[x_to_screen(screen, x1)],
        &[y_to_screen(screen, y1)],
        &[0.0],
        &[x_to_screen(screen, x2)],
        &[y_to_screen(screen, y2)],
        &[0.0],
    )?;
    screen.draw(&line)
}

pub fn point(screen: &mut Screen, x: f64, y: f64) -> Result<(), ProcessingErr> {
    let point = Point::new(
        screen,
        &[x_to_screen(screen, x)],
        &[y_to_screen(screen, y)],
        &[0.0],
    )?;
    screen.draw(&point)
}

pub fn rect(screen: &mut Screen, x: f64, y: f64, w: f64, h: f64) -> Result<(), ProcessingErr> {
    let rect = Rect::new(
        screen,
        &[x_to_screen(screen, x)],
        &[y_to_screen(screen, y)],
        &[0.0],
        &[w_to_screen(screen, w)],
        &[h_to_screen(screen, h)],
    )?;
    screen.draw(&rect)
}

pub fn ellipse(screen: &mut Screen, x: f64, y: f64, w: f64, h: f64) -> Result<(), ProcessingErr> {
    let ellipse = Ellipse::new(
        screen,
        &[x_to_screen(screen, x)],
        &[y_to_screen(screen, y)],
        &[0.0],
        &[w_to_screen(screen, w)],
        &[h_to_screen(screen, h)],
    )?;
    screen.draw(&ellipse)
}
