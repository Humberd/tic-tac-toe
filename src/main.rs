use crate::board::{Board, Coords};
use crate::player::Player;
use crate::screen_drawer::ScreenDrawer;
use std::cell::RefCell;
use std::io::{stdin, stdout, Write};

mod board;
mod screen_drawer;
mod player;

fn main() -> Result<(), std::io::Error> {
    let mut board = RefCell::from(Board::new());
    let mut screen_drawer = ScreenDrawer::new(&board);

    let mut buffer = String::new();

    loop {
        screen_drawer.draw();
        print!("Coords: ");
        stdout().flush();
        stdin().read_line(&mut buffer);


        let coords_str: Vec<&str> = buffer.trim().split(',').collect();
        let coords: Coords = (coords_str[0].parse().unwrap(), coords_str[1].parse().unwrap());
        board.borrow_mut().make_move(coords, Player::X)?;

        buffer.clear();
    }

    // board.borrow_mut().make_move((0, 1), Player::X);
    //
    // screen_drawer.draw()?;
    //
    // board.borrow_mut().make_move((2, 1), Player::O);
    //
    // screen_drawer.draw()?;

    Ok(())
}

