use std::cell::RefCell;
use std::rc::Rc;

use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

#[derive(Debug)]
struct Perceptron {
    weights: Vec<f32>,
    c: f32, // learning constant
}

impl Perceptron {
    fn new(n: usize) -> Self {
        let mut rng = rand::thread_rng();

        // start with random weights
        let mut weights = Vec::with_capacity(n);
        for _ in 0..weights.capacity() {
            weights.push(rng.gen_range(-1.0..1.0));
        }

        Self { weights, c: 0.01 }
    }

    fn feedforward(&self, inputs: impl AsRef<[f32]>) -> isize {
        let inputs = inputs.as_ref();
        assert_eq!(self.weights.len(), inputs.len());

        let mut sum = 0.0;
        for i in 0..self.weights.len() {
            sum += inputs[i] * self.weights[i];
        }
        self.activate(sum)
    }

    fn activate(&self, sum: f32) -> isize {
        if sum > 0.0 {
            1
        } else {
            -1
        }
    }

    fn train(&mut self, inputs: impl AsRef<[f32]>, desired: isize) {
        let guess = self.feedforward(&inputs);
        let error = desired - guess;
        for i in 0..self.weights.len() {
            self.weights[i] = self.c * error as f32 * inputs.as_ref()[i] as f32;
        }
    }
}

struct Trainer {
    inputs: [f32; 3],
    answer: isize,
}

impl Trainer {
    fn new(x: f32, y: f32, a: isize) -> Self {
        Self {
            // 3rd point is input bias
            inputs: [x, y, 1.0],
            answer: a,
        }
    }
}

fn f(x: f32) -> f32 {
    2.0 * x + 1.0
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen, _: f64) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    core::translate(
        screen,
        screen.width() as f64 / 2.0,
        screen.height() as f64 / 2.0,
    );

    // TODO:

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let ptron = Rc::new(RefCell::new(None));
    let training = Rc::new(RefCell::new(None));

    core::run(
        || {
            let mut rng = rand::thread_rng();

            let screen = setup()?;

            let p = Perceptron::new(3);
            *ptron.borrow_mut() = Some(p);

            let mut t = vec![];
            for _ in 0..2000 {
                let x = rng.gen_range(-(screen.width() as f32) / 2.0..screen.width() as f32 / 2.0);
                let y =
                    rng.gen_range(-(screen.height() as f32) / 2.0..screen.height() as f32 / 2.0);
                let answer = if y < f(x) { -1 } else { 1 };
                t.push(Trainer::new(x, y, answer));
            }
            *training.borrow_mut() = Some(t);

            Ok(screen)
        },
        draw,
    )?;

    Ok(())
}
