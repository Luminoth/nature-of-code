use std::cell::RefCell;
use std::rc::Rc;

use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 240)
}

fn draw(screen: &mut Screen, random_counts: &mut Vec<u32>) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..random_counts.len());
    random_counts[index] += 1;

    core::stroke_grayscale(screen, 0.0);
    core::fill_grayscale(screen, 175.0);

    let w = screen.width() / random_counts.len() as u32;
    for (x, v) in random_counts.iter().enumerate() {
        core::shapes::rect(
            screen,
            (x as u32 * w) as f64,
            (screen.height() - v) as f64,
            (w - 1) as f64,
            *v as f64,
        )?;
    }

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let random_counts = Rc::new(RefCell::new(vec![0; 20]));

    core::run(setup, |screen, _| {
        draw(screen, random_counts.borrow_mut().as_mut())
    })?;

    Ok(())
}
