use crate::board::{Board, Coords};
use crate::player::Player;
use crate::screen_drawer::ScreenDrawer;
use std::cell::RefCell;

mod board;
mod screen_drawer;
mod player;

fn main() -> Result<(), std::io::Error> {
    let mut board = RefCell::from(Board::new());
    let mut screen_drawer = ScreenDrawer::new(&board);

    board.borrow_mut().make_move((0,1), Player::X);

    screen_drawer.draw()?;

    board.borrow_mut().make_move((2,1), Player::O);

    screen_drawer.draw()?;

    Ok(())
}

