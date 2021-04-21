use once_cell::sync::OnceCell;
use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

static mut RANDOM_COUNTS: OnceCell<Vec<u32>> = OnceCell::new();

fn random_counts() -> &'static mut Vec<u32> {
    unsafe { RANDOM_COUNTS.get_mut().unwrap() }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    let screen = core::create_canvas(640, 240)?;

    unsafe {
        RANDOM_COUNTS.set(vec![0; 20]).unwrap();
    }

    Ok(screen)
}

fn draw(screen: &mut Screen) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    let random_counts = random_counts();

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
    core::run(setup, draw)?;

    Ok(())
}
