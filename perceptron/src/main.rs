use std::cell::RefCell;
use std::rc::Rc;

use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

// lower learning constant produces a slower,
// more visually interesting solution
// (default is 0.01)
const C: f32 = 0.00001;

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

        Self { weights, c: C }
    }

    fn feedforward(&self, inputs: impl AsRef<[f32]>) -> isize {
        let inputs = inputs.as_ref();
        assert_eq!(self.weights.len(), inputs.len());

        let mut sum = 0.0;
        for (i, input) in inputs.iter().enumerate().take(self.weights.len()) {
            sum += input * self.weights[i];
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
        for (i, input) in inputs.as_ref().iter().enumerate().take(self.weights.len()) {
            self.weights[i] += self.c * error as f32 * input;
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

fn draw(
    screen: &mut Screen,
    _: f64,
    ptron: &mut Perceptron,
    training: impl AsRef<[Trainer]>,
    current: &mut usize,
) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    let hw = screen.width() as f64 / 2.0;
    let hh = screen.width() as f64 / 2.0;

    core::translate(screen, hw, hh);

    // draw the target line
    screen.stroke_weight(4.0);
    core::stroke_grayscale(screen, 127.0);
    core::shapes::line(screen, -hw, f(-hw as f32) as f64, hw, f(hw as f32) as f64)?;

    // draw the line based on the current weights
    // formula is weights[0]*x + weights[1]*y + weights[2] = 0
    screen.stroke_weight(1.0);
    core::stroke_grayscale(screen, 0.0);
    core::shapes::line(
        screen,
        -hw,
        ((-ptron.weights[2] - ptron.weights[0] * -hw as f32) / ptron.weights[1]) as f64,
        hw,
        ((-ptron.weights[2] - ptron.weights[0] * hw as f32) / ptron.weights[1]) as f64,
    )?;

    let training = training.as_ref();
    ptron.train(training[*current].inputs, training[*current].answer);
    *current = (*current + 1) % training.len();

    for trainer in training.iter().take(*current) {
        core::stroke_grayscale(screen, 0.0);
        let guess = ptron.feedforward(trainer.inputs);
        if guess > 0 {
            screen.fill_off();
        } else {
            core::fill_grayscale(screen, 0.0);
        }

        core::shapes::ellipse(
            screen,
            trainer.inputs[0] as f64,
            trainer.inputs[1] as f64,
            8.0,
            8.0,
        )?;
    }

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let mut ptron = Perceptron::new(3);
    let training = Rc::new(RefCell::new(None));
    let mut current = 0;

    core::run(
        || {
            let mut rng = rand::thread_rng();

            let screen = setup()?;

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
        |screen, dt| {
            draw(
                screen,
                dt,
                &mut ptron,
                training.borrow().as_ref().unwrap(),
                &mut current,
            )?;

            Ok(())
        },
    )?;

    Ok(())
}
