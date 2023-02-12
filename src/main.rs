//use crossterm::{queue, style::Print};
//use std::io::{stdout, Write};
use tic_tac_toe::*;

fn main() {
    let mut board = Board::new();

    println!("-------------");
    for row in board.cells {
        for cell in row {
            print!("|");
            match cell {
                Some(player) => print!("{}", player),
                None => print!("   "),
            }
        }
        print!("|");
        println!("\n-------------");
    }
}
