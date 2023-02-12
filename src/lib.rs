use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    X,
    O,
}

pub struct Board {
    cells: [[Option<Player>; 3]; 3],
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

    pub fn print_board(&self) {
        println!("-------------");
        for row in self.cells {
            for cell in row {
                print!("|");
                match cell {
                    Some(player) => print!("{player}"),
                    None => print!("   "),
                }
            }
            print!("|");
            println!("\n-------------");
        }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<Player> {
        self.cells[row][col]
    }

    pub fn set_cell(&mut self, row: usize, col: usize) -> Result<(), String> {
        if self.cells[row][col].is_some() {
            return Err("Cell is already occupied".to_string());
        }
        self.cells[row][col] = Some(self.turn);
        self.turn = match self.turn {
            Player::X => Player::O,
            Player::O => Player::X,
        };
        Ok(())
    }

    pub fn get_winner(&self) -> Option<Player> {
        self.winner
    }

    pub fn set_winner(&mut self, winner: Option<Player>) {
        self.winner = winner;
    }

    pub fn check_winner(&self) -> Option<Player> {
        let mut winner = None;
        for row in self.cells {
            if row[0] == row[1] && row[1] == row[2] {
                winner = row[0];
            }
        }
        for col in 0..3 {
            if self.cells[0][col] == self.cells[1][col] && self.cells[1][col] == self.cells[2][col]
            {
                winner = self.cells[0][col];
            }
        }
        if self.cells[0][0] == self.cells[1][1] && self.cells[1][1] == self.cells[2][2] {
            winner = self.cells[0][0];
        }
        if self.cells[0][2] == self.cells[1][1] && self.cells[1][1] == self.cells[2][0] {
            winner = self.cells[0][2];
        }
        winner
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Player::X => "X",
                Player::O => "O",
            }
        )
    }
}
