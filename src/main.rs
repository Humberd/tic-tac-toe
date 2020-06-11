use console::Term;
use crate::board::{Board, Coords};
use std::fmt;
use crate::player::Player;

mod board;
mod player;

fn main() -> Result<(), std::io::Error> {
    let mut board = Board::new();
    let possible_moves = board.get_possible_moves();
    println!("{:?}", possible_moves);

    board.make_move((0, 0), Player::X)?;

    let possible_moves = board.get_possible_moves();
    println!("{:?}", possible_moves);

    board.undo()?;

    let possible_moves = board.get_possible_moves();
    println!("{:?}", possible_moves);

    let term = Term::stdout();
    term.clear_screen();

    Ok(())
}

