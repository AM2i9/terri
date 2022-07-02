use crate::board::Board;

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board{
                height: 10,
                width: 12
            }
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }
}