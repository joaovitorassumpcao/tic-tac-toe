use ndarray::prelude::*;

#[derive(Copy, Clone)]
enum State {
    X,
    O,
    Nil,
}

#[derive(Copy, Clone)]
struct Cell {
    state: State,
}

pub struct Board {
    cells: Array2<Cell>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
			cells: Array2::from_elem((3, 3), Cell { state: State::Nil }),
        }
    }

	pub fn get_cell_state(&self, x: usize, y: usize) -> String {
		format!("{}", self.cells[[x, y]].state)
	}
}


impl std::fmt::Display for Board {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for row in self.cells.outer_iter() {
			for cell in row {
				match cell.state {
					State::X => write!(f, "X")?,
					State::O => write!(f, "O")?,
					State::Nil => write!(f, " ")?,
				}
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

impl std::fmt::Display for Cell {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self.state {
			State::X => write!(f, "X"),
			State::O => write!(f, "O"),
			State::Nil => write!(f, " "),
		}
	}
}

impl std::fmt::Display for State {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			State::X => write!(f, "X"),
			State::O => write!(f, "O"),
			State::Nil => write!(f, " "),
		}
	}
}