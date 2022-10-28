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
}
