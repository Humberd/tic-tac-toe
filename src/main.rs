use crate::board::{Board, Coords, State};
use crate::player::Player;
use crate::screen_drawer::ScreenDrawer;
use std::cell::RefCell;
use std::io::{stdin, stdout, Write, Error};
use std::ops::DerefMut;

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
        board.borrow_mut().make_move(coords, &Player::X)?;

        make_best_move(board.borrow_mut().deref_mut());

        buffer.clear();
    }

    Ok(())
}

fn make_best_move(board: &mut Board) -> Result<(), Error> {
    let mut best_score = -9999i32;
    let mut best_move: Option<Coords> = None;

    let bot = Player::O;

    for coords in board.get_possible_moves() {
        board.make_move(coords, &bot);
        let mut score = minimax(false, &bot, board)?;
        board.undo();
        if score > best_score {
            best_score = score;
            best_move = Some(coords)
        }
    }

    return board.make_move(best_move.unwrap(), &bot);
}

fn minimax(is_max_turn: bool, player: &Player, board: &mut Board) -> Result<i32, Error> {
    let state = board.get_state();

    match state {
        State::Draw => return Ok(0),
        State::Over(winner) => return if winner == *player { Ok(1) } else { Ok(-1) },
        State::InProgress => ()
    }

    let mut scores: Vec<i32> = vec![];
    for coords in board.get_possible_moves() {
        board.make_move(coords, player)?;
        scores.push(minimax(!is_max_turn, player, board).unwrap());
        board.undo()?;
    }

    return if is_max_turn {
        Ok(*scores.iter().max().unwrap())
    } else {
        Ok(*scores.iter().min().unwrap())
    };
}
