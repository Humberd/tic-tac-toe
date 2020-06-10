use console::Term;
use crate::board::{Board, Coords};
use std::fmt;
use crate::player::Player;

mod board;
mod player;

fn main() {
    let mut board = Board::new();
    let possible_moves = board.get_possible_moves();
    println!("{:?}", possible_moves);

    let coords: Coords = [0, 0];
    board.make_move(&coords, Player::X);

    let possible_moves = board.get_possible_moves();
    println!("{:?}", possible_moves);

    let term = Term::stdout();
    term.clear_screen();
}

