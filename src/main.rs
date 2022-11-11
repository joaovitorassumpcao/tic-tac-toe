use std::io::{Write, stdout};
use crossterm::{queue, style::Print};
use tic_tac_toe::*;

fn main() {
    let mut stdout = stdout();   
    let mut board = Board::new();
    
    queue!(stdout, Print(board)).expect("print board.");
    stdout.flush().expect("flush to stdout before loop.");

    loop {
        
    }
}
