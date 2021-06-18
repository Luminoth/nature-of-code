#![allow(dead_code)]

// TODO: finish this

use processing::errors::ProcessingErr;
use processing::Screen;

#[derive(Debug)]
struct CA {
    cells: Vec<i8>,
    ruleset: [i8; 8],
    cell_size: f64,
    generation: u32,
}

impl CA {
    fn new(screen: &Screen) -> Self {
        let len = screen.width() as usize;
        let mut cells = vec![0; len];
        cells[len / 2] = 1;

        let ruleset = [0, 1, 0, 1, 1, 0, 1, 0];

        Self {
            cells,
            ruleset,
            cell_size: 10.0,
            generation: 0,
        }
    }

    fn generate(&mut self) {
        let mut next_gen = Vec::with_capacity(self.cells.len());
        for i in 1..self.cells.len() - 1 {
            let left = self.cells[i - 1];
            let me = self.cells[i];
            let right = self.cells[i + 1];
            next_gen[i] = self.rules(left, me, right);
        }
        self.cells = next_gen;

        self.generation += 1;
    }

    fn rules(&self, a: i8, b: i8, c: i8) -> i8 {
        let s = format!("{}{}{}", a, b, c);
        let index = s.parse::<usize>().unwrap();
        self.ruleset[index]
    }

    fn draw(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        for i in 0..self.cells.len() {
            if self.cells[i] == 1 {
                core::fill_grayscale(screen, 0.0);
            } else {
                core::fill_grayscale(screen, 255.0);
            }

            core::shapes::rect(
                screen,
                i as f64 * self.cell_size,
                self.generation as f64 * self.cell_size,
                self.cell_size,
                self.cell_size,
            )?;
        }

        Ok(())
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen, _dt: f64) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    core::run(setup, draw)?;

    Ok(())
}
