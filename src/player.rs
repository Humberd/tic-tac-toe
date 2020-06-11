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
}