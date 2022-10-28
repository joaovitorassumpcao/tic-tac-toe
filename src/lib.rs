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
    cells: [[[Cell; 3]; 3]; 3],
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [[[Cell { state: State::Nil }; 3]; 3]; 3],
        }
    }
}
