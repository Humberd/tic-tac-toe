use std::io::{ErrorKind};
use crate::player::Player;

pub struct Board {
    pub fields: [[Player; 3]; 3],
    undo_stack: Vec<Coords>,
}

pub type Coords = (usize, usize);

#[derive(Debug)]
pub enum State {
    InProgress,
    Draw,
    Over(Player),
}

impl Board {
    pub fn new() -> Board {
        return Board {
            fields: [
                [Player::None, Player::None, Player::None],
                [Player::None, Player::None, Player::None],
                [Player::None, Player::None, Player::None],
            ],
            undo_stack: vec![],
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
        if coords.0 >= 3 {
            return Err(ErrorKind::InvalidInput);
        }

        if coords.1 >= 3 {
            return Err(ErrorKind::InvalidInput);
        }

        self.fields[coords.0][coords.1] = player;
        self.undo_stack.push(coords);

        return Ok(());
    }

    pub fn undo(&mut self) -> Result<(), ErrorKind> {
        let coords = match self.undo_stack.pop() {
            Some(coords) => coords,
            None => return Err(ErrorKind::InvalidData),
        };

        self.fields[coords.0][coords.1] = Player::None;

        return Ok(());
    }

    pub fn get_state(&mut self) -> State {
        // rows
        for row_index in 0..3 {
            let row_sum: i32 = self.fields[row_index].iter().map(|player| player.as_number()).sum();
            if i32::abs(row_sum) == 3 {
                return State::Over(self.get_player(row_sum));
            }
        }

        // columns
        for column_index in 0..3 {
            let columns = [&self.fields[0][column_index], &self.fields[1][column_index], &self.fields[2][column_index]];
            let row_sum: i32 = columns.iter().map(|player| player.as_number()).sum();
            if i32::abs(row_sum) == 3 {
                return State::Over(self.get_player(row_sum));
            }
        }

        //diagonal
        {
            let diag1 = [&self.fields[0][0], &self.fields[1][1], &self.fields[2][2]];
            let diag2 = [&self.fields[0][2], &self.fields[1][1], &self.fields[2][0]];

            let row_sum: i32 = diag1.iter().map(|player| player.as_number()).sum();
            if i32::abs(row_sum) == 3 {
                return State::Over(self.get_player(row_sum));
            }

            let row_sum: i32 = diag2.iter().map(|player| player.as_number()).sum();
            if i32::abs(row_sum) == 3 {
                return State::Over(self.get_player(row_sum));
            }
        }


        let none_player_exists = self.fields.iter().flatten().any(|player| player == &Player::None);

        if none_player_exists {
            return State::InProgress;
        }

        return State::Draw;
    }

    fn get_player(&self, value: i32) -> Player {
        if (value > 0) {
            return Player::X;
        }

        return Player::O;
    }
}