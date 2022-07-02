use crate::tetriminos::{Block, TetriminoShapes};

pub struct Board {
    pub height: u32,
    pub width: u32,
    pub active_blocks: [Block; 4],
    pub settled_blocks: Vec<Block>,
}

impl Board {
    pub fn new(height: u32, width: u32) -> Board {
        let first_blocks = TetriminoShapes::L.blocks();
        
        let mut new_board = Board {
            height,
            width,
            active_blocks: first_blocks,
            settled_blocks: Vec::new(),
        };

        let (start_x, start_y) = new_board.get_start_pos();
        
        for block in &mut new_board.active_blocks.iter_mut() {
            block.x += start_x;
            block.y += start_y;
        }

        new_board
    }
    
    pub fn edge_blocks_down(&mut self) {
        for block in &mut self.active_blocks.iter_mut() {
            block.y += 1;
        }
    }

    pub fn move_block_right(&mut self) {
        for block in &mut self.active_blocks.iter_mut() {
            if block.x < self.width {
                block.x += 1;
            }
        }
    }

    pub fn move_block_left(&mut self) {
        for block in &mut self.active_blocks.iter_mut() {
            if block.x > 0 {
                block.x -= 1;
            }
        }
    }

    fn block_at_coord(&self, x: u32, y: u32) -> bool {
        for block in self.active_blocks.iter() {
            if block.x == x && block.y == y {
                return true;
            }
        }
        false
    }

    pub fn draw(&self) -> String {
        let mut board = String::new();
        for y in 0..self.height {
            for _ in 0..2 {
                for x in 0..self.width {
                    if self.block_at_coord(x, y) {
                        board.push_str("###");
                    } else {
                        board.push_str("   ");
                    }
                }
                board.push_str("\n");
            }
        }
        board
    }

    pub fn get_start_pos(&self) -> (u32, u32) {
        (self.width / 2, 0)
    }
}