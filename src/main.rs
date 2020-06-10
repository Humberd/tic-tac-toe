use console::Term;
use crate::board::Board;
use std::fmt;

mod board;

fn main() {
    let board = Board::new();
    let possible_moves = board.get_possible_moves();
    println!("{:?}", possible_moves);

    let term = Term::stdout();
    term.clear_screen();

}

