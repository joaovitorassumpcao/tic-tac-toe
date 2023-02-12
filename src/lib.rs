use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    X,
    O,
}

pub struct Board {
    pub cells: [[Option<Player>; 3]; 3],
    turn: Player,
    winner: Option<Player>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    /// Creates a new board with all cells in the Nil state.
    pub fn new() -> Board {
        Board {
            cells: [[None; 3]; 3],
            turn: Player::X,
            winner: None,
        }
    }


}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Player::X => "X",
                Player::O => "O",
            }
        )
    }
}

