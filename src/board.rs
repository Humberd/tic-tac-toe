
#[derive(PartialEq)]
pub enum FieldState {
    X,
    O,
    None,
}

impl FieldState {
    fn as_number(&self) -> i32 {
        return match self {
            FieldState::X => 1,
            FieldState::O => -1,
            FieldState::None => 0,
        }
    }
}

pub struct Board {
    fields: [[FieldState; 3]; 3],
}


impl Board {
    pub fn new() -> Board {
        return Board {
            fields: [
                [FieldState::None, FieldState::None, FieldState::None],
                [FieldState::None, FieldState::None, FieldState::None],
                [FieldState::None, FieldState::None, FieldState::None],
            ]
        };
    }

    pub fn get_possible_moves(&self) -> Vec<[usize; 2]> {
        let mut possible_moves: Vec<[usize; 2]> = vec![];


        for row_index in 0..3 {
            for column_index in 0..3 {
                if self.fields[row_index][column_index] == FieldState::None {
                    possible_moves.push([row_index, column_index])
                }
            }
        }

        return possible_moves;
    }
}