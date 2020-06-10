use std::io::{ErrorKind};

#[derive(PartialEq)]
pub enum Player {
    X,
    O,
    None,
}

impl Player {
    fn as_number(&self) -> i32 {
        return match self {
            Player::X => 1,
            Player::O => -1,
            Player::None => 0,
        };
    }
}

pub struct Board {
    fields: [[Player; 3]; 3],
}

pub type Coords = [usize; 2];


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
                    possible_moves.push([row_index, column_index])
                }
            }
        }

        return possible_moves;
    }

    pub fn make_move(&mut self, coords: &Coords, player: Player) -> Result<(), ErrorKind> {
        if coords[0] >= 3 || coords[0] < 0 {
            return Result::Err(ErrorKind::InvalidInput)
        }

        if coords[1] >= 3 || coords[1] < 0 {
            return Result::Err(ErrorKind::InvalidInput)
        }

        self.fields[coords[0]][coords[1]] = player;

        return Result::Ok(());
    }
}