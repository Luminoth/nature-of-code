use processing::errors::ProcessingErr;
use processing::Screen;

#[derive(Debug)]
struct CA {
    cells: Vec<i8>,
    ruleset: [i8; 8],
}

impl CA {
    fn new(screen: &Screen) -> Self {
        let len = screen.width() as usize;
        let mut cells = vec![0; len];
        cells[len / 2] = 1;

        let ruleset = [0, 1, 0, 1, 1, 0, 1, 0];

        Self { cells, ruleset }
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
    }

    fn rules(&self, a: i8, b: i8, c: i8) -> i8 {
        let s = format!("{}{}{}", a, b, c);
        let index = s.parse::<usize>().unwrap();
        self.ruleset[index]
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
