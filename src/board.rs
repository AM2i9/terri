use crate::tetrominos::{Block, Tetromino};

pub struct Board {
    pub height: u32,
    pub width: u32,
    pub active_blocks: [[u32; 2]; 4],
    pub settled_blocks: Box<[Option<Tetromino>]>,
}

impl Board {
    pub fn new(height: u32, width: u32) -> Board {
        let first_blocks = Tetromino::L.blocks();

        let max_blocks = (height * width) as usize;
        
        let mut new_board = Board {
            height,
            width,
            active_blocks: first_blocks,
            settled_blocks: vec![None; max_blocks].into_boxed_slice(),
        };

        let (start_x, start_y) = new_board.get_start_pos();
        
        for block in &mut new_board.active_blocks.iter_mut() {
            block[0] += start_x;
            block[1] += start_y;
        }

        new_board
    }

    pub fn new_active_blocks(&mut self) {
        self.active_blocks = Tetromino::L.blocks();

        let (start_x, start_y) = self.get_start_pos();
        
        for block in &mut self.active_blocks.iter_mut() {
            block[0] += start_x;
            block[1] += start_y;
        }
    }

    pub fn settle_blocks(&mut self) {
        for block in &self.active_blocks {
            self.settled_blocks[((self.width * block[1]) + block[0]) as usize] = Some(Tetromino::L);
        }
    }
    
    pub fn edge_blocks_down(&mut self) {
        for block in &mut self.active_blocks.iter_mut() {
            block[1] += 1;
        }
    }

    pub fn drop_blocks(&mut self) {
        while !self.blocks_will_collide_down() {
            self.edge_blocks_down();
        }
    }

    pub fn move_blocks_right(&mut self) {
        for block in &mut self.active_blocks.iter_mut() {
            block[0] += 1;
        }
    }

    pub fn move_blocks_left(&mut self) {
        for block in &mut self.active_blocks.iter_mut() {
            block[0] -= 1;
        }
    }

    pub fn blocks_will_collide_down(&self) -> bool{
        for block in self.active_blocks.iter() {
            if block[1] == self.height - 1 || self.settled_block_at_coord(block[0], block[1] + 1) {
                return true;
            }
        }
        false
    }

    pub fn will_collide_left(&self) -> bool {
        for block in self.active_blocks.iter() {
            if block[0] == 0 || self.settled_block_at_coord(block[0] - 1, block[1]) {
                return true;
            }
        }
        false
    }

    pub fn will_collide_right(&self) -> bool {
        for block in self.active_blocks.iter() {
            if block[0] >= self.width - 1  || self.settled_block_at_coord(block[0] + 1, block[1]) {
                return true;
            }
        }
        false
    }

    fn block_at_coord(&self, x: u32, y: u32) -> bool {
        for block in self.active_blocks.iter() {
            if block[0] == x && block[1] == y {
                return true;
            }
        }

        self.settled_block_at_coord(x, y)
    }

    fn settled_block_at_coord(&self, x: u32, y: u32) -> bool {
        self.settled_blocks[((self.width * y) + x) as usize].is_some()
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
        ((self.width / 2 )- 1, 0)
    }
}