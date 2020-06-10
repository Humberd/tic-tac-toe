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