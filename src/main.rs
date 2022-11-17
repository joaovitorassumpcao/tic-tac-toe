use crossterm::{queue, style::Print};
use std::io::{stdout, Write};
use tic_tac_toe::*;

fn main() {
    let mut stdout = stdout().lock();
    let mut board = Board::new();

    queue!(stdout, board);
    stdout.flush().expect("flush to stdout before loop.");
    write!()
    //loop {

    //}
}
