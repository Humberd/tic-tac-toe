use std::fmt::{Display, Formatter, Error};

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Player {
    X,
    O,
    None,
}

impl Player {
    pub fn as_number(&self) -> i32 {
        return match self {
            Player::X => 1,
            Player::O => -1,
            Player::None => 0,
        };
    }

    pub fn as_char(&self) -> char {
        return match self {
            Player::X => 'X',
            Player::O => 'O',
            Player::None => ' ',
        };
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.as_char())
    }
}