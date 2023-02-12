//use crossterm::{queue, style::Print};
//use std::io::{stdout, Write};
use tic_tac_toe::*;

fn main() {
    let mut board = Board::new();

    while board.get_winner().is_none() {
        board.print_board();
        println!("Enter row and column: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut input = input.split_whitespace();
        let row: usize = input.next().unwrap().parse().unwrap();
        let col: usize = input.next().unwrap().parse().unwrap();
        board.set_cell(row, col).unwrap();
        board.set_winner(board.check_winner());
    }
    println!("Winner is: {:?}", board.check_winner().unwrap());
}
