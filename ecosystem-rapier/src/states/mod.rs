//! Game states

pub mod game;

/// The game state
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Game,
}
