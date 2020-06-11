use crate::board::Board;
use console::Term;
use std::io::{Write, Error};
use std::cell::RefCell;

pub struct ScreenDrawer<'a> {
    board: &'a RefCell<Board>,
    term: Term
}

impl<'a> ScreenDrawer<'a> {
    pub fn new(board: &RefCell<Board>) -> ScreenDrawer {
        ScreenDrawer{
            board: board,
            term: Term::stdout()
        }
    }

    pub fn draw(&self) -> Result<(), Error> {
        self.term.clear_screen()?;

        for (index, row) in self.board.borrow().fields.iter().enumerate() {
            println!("{} | {} | {}", row[0], row[1], row[2]);

            if (index < 2) {
                println!("---------")
            }
        }

        Ok(())
    }
}