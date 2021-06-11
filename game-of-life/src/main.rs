use std::cell::RefCell;
use std::rc::Rc;

use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

#[derive(Debug, Default, Copy, Clone)]
struct LifeCell {
    state: i8,
    previous_state: i8,
}

impl LifeCell {
    fn new() -> Self {
        let mut rng = rand::thread_rng();

        let state = rng.gen_range(0..=1);

        Self {
            state,
            previous_state: state,
        }
    }

    fn is_alive(&self) -> bool {
        self.state == 1
    }
}

#[derive(Debug)]
struct GameOfLife {
    columns: usize,
    rows: usize,

    board: Vec<Vec<LifeCell>>,
    cell_size: f64,
}

impl GameOfLife {
    fn new(screen: &Screen) -> Self {
        let cell_size = 10;

        let columns = (screen.width() / cell_size) as usize;
        let rows = (screen.height() / cell_size) as usize;

        // TODO: is there a better way to init a 2d vec?
        let mut board = vec![vec![LifeCell::default(); rows]; columns];
        for x in 0..columns {
            for y in 0..rows {
                board[x][y] = LifeCell::new();
            }
        }

        Self {
            columns,
            rows,
            board,
            cell_size: cell_size as f64,
        }
    }

    fn generate(&mut self) {
        // copy the current state to the previous state
        for row in self.board.iter_mut() {
            for cell in row.iter_mut() {
                cell.previous_state = cell.state;
            }
        }

        for x in 1..self.columns - 1 {
            for y in 1..self.rows - 1 {
                // add up the neighbor states to get the count of live neighbors
                let mut neighbors = 0;
                // TODO: not sure how to iterate a negative range
                for i in 0..=2 {
                    for j in 0..=2 {
                        neighbors += self.board[x + i - 1][y + j - 1].previous_state;
                    }
                }

                // remove the current cell's value that was added in
                neighbors -= self.board[x][y].previous_state;

                // set the cell's new state
                self.board[x][y].state = if self.board[x][y].is_alive() && neighbors < 2 {
                    // lonely death
                    0
                } else if self.board[x][y].is_alive() && neighbors > 3 {
                    // overpopulation death
                    0
                } else if !self.board[x][y].is_alive() && neighbors == 3 {
                    // birth
                    1
                } else {
                    // stasis
                    self.board[x][y].previous_state
                };
            }
        }
    }

    fn draw(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        for x in 0..self.columns {
            for y in 0..self.rows {
                if self.board[x][y].is_alive() {
                    core::fill_grayscale(screen, 0.0);
                } else {
                    core::fill_grayscale(screen, 255.0);
                }
                //core::stroke_grayscale(screen, 0.0);

                core::shapes::rect(
                    screen,
                    x as f64 * self.cell_size,
                    y as f64 * self.cell_size,
                    self.cell_size,
                    self.cell_size,
                )?;
            }
        }

        Ok(())
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen, _dt: f64, game_of_life: &mut GameOfLife) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    game_of_life.generate();
    game_of_life.draw(screen)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let game_of_life = Rc::new(RefCell::new(None));

    core::run(
        || {
            let screen = setup()?;

            *game_of_life.borrow_mut() = Some(GameOfLife::new(&screen));

            Ok(screen)
        },
        |screen, dt| draw(screen, dt, game_of_life.borrow_mut().as_mut().unwrap()),
    )?;

    Ok(())
}
