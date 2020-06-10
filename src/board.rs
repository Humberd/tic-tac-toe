use std::io::{ErrorKind};
use crate::player::Player;


pub struct Board {
    fields: [[Player; 3]; 3],
}

pub type Coords = (usize, usize);

impl Board {
    pub fn new() -> Board {
        return Board {
            fields: [
                [Player::None, Player::None, Player::None],
                [Player::None, Player::None, Player::None],
                [Player::None, Player::None, Player::None],
            ]
        };
    }

    pub fn get_possible_moves(&self) -> Vec<Coords> {
        let mut possible_moves: Vec<Coords> = vec![];


        for row_index in 0..3 {
            for column_index in 0..3 {
                if self.fields[row_index][column_index] == Player::None {
                    possible_moves.push((row_index, column_index))
                }
            }
        }

        return possible_moves;
    }

    pub fn make_move(&mut self, coords: Coords, player: Player) -> Result<(), ErrorKind> {
        if coords.0 >= 3 || coords.0 < 0 {
            return Result::Err(ErrorKind::InvalidInput)
        }

        if coords.1 >= 3 || coords.1 < 0 {
            return Result::Err(ErrorKind::InvalidInput)
        }

        self.fields[coords.0][coords.1] = player;

        return Result::Ok(());
    }
}