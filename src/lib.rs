use ndarray::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    X,
    O,
    Nil,
}

#[derive(Copy, Clone, PartialEq)]
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

impl Cell {
    pub fn get(&self) -> State {
        self.state
    }

    pub fn set(&mut self, state: State) {
        self.state = state;
    }
}

/// Board is a 3x3 grid of cells. Each cell can be in one of three states: X, O, or Nil.
/// Nil is the default state.
/// X and O are the states of the two players.
/// The board is initialized with all cells in the Nil state.
impl Board {
	/// Creates a new board with all cells in the Nil state.
    pub fn new() -> Board {
        Board {
            cells: Array2::from_elem((3, 3), Cell { state: State::Nil }),
        }
    }

	/// Returns the state of the cell at the given coordinates.
    pub fn get_cell_state(&self, x: usize, y: usize) -> State {
        self.cells[[x, y]].get()
    }

	/// Sets the state of the cell at the given coordinates.
    pub fn set_cell_state(&mut self, x: usize, y: usize, state: State) {
        self.cells[[x, y]].set(state);
    }

	/// Returns the row at the given index.
    pub fn get_row(&self, y: usize) -> Vec<State> {
        let mut row = Vec::new();
        for x in 0..3 {
            row.push(self.get_cell_state(x, y));
        }
        row
    }

	/// Returns the column at the given index.
    pub fn get_column(&self, x: usize) -> Vec<State> {
        let mut column = Vec::new();
        for y in 0..3 {
            column.push(self.get_cell_state(x, y));
        }
        column
    }

    pub fn is_full(&self) -> bool {
        !self.cells.iter().any(|&x| x.state == State::Nil)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.cells.outer_iter() {
            for cell in row {
                match cell.state {
                    State::X => write!(f, " X ")?,
                    State::O => write!(f, " O ")?,
                    State::Nil => write!(f, "   ")?,
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
