use crate::board::Board;

pub struct Game {
    board: Board,
    down_counter: u32
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(12, 12),
            down_counter: 0
        }
    }

    pub fn get_board(&mut self) -> &mut Board {
        &mut self.board
    }

    pub fn update(&mut self) {
        if self.down_counter > 2 {
            self.board.edge_blocks_down();
            self.down_counter = 0;
        } else {
            self.down_counter += 1;
        }
    }
}