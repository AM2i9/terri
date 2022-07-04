use crate::tetrominos::{Tetromino, TetrominoShape};

pub struct Board {
    pub height: i32,
    pub width: i32,
    pub active_tetrmomino: Tetromino,
    pub active_blocks: [[i32; 2]; 4],
    pub settled_blocks: Box<[Option<TetrominoShape>]>,
}

impl Board {
    pub fn new(height: i32, width: i32) -> Board {
        let first_mino = Tetromino::random();
        let first_blocks = first_mino.blocks();

        let max_blocks = (height * width) as usize;
        
        let mut new_board = Board {
            height,
            width,
            active_tetrmomino: first_mino,
            active_blocks: first_blocks,
            settled_blocks: vec![None; max_blocks].into_boxed_slice(),
        };

        let (start_x, start_y) = new_board.get_start_pos();

        new_board.active_tetrmomino.x = start_x;
        new_board.active_tetrmomino.y = start_y;
        
        for block in &mut new_board.active_blocks.iter_mut() {
            block[0] += start_x;
            block[1] += start_y;
        }

        new_board
    }

    pub fn new_active_blocks(&mut self) {
        let new_mino = Tetromino::random();
        self.active_blocks = new_mino.blocks();
        self.active_tetrmomino = new_mino;

        let (start_x, start_y) = self.get_start_pos();

        self.active_tetrmomino.x = start_x;
        self.active_tetrmomino.y = start_y;
        
        for block in &mut self.active_blocks.iter_mut() {
            block[0] += start_x;
            block[1] += start_y;
        }
    }

    pub fn rotate_blocks_cw(&mut self) {
        self.active_tetrmomino.rotate_cw();
        let mut new_blocks = self.active_tetrmomino.blocks();

        for block in new_blocks.iter_mut() {
            if block[0] < 0 {
                self.active_tetrmomino.x += 1;
            } else if block[0] >= self.width {
                self.active_tetrmomino.x -= 1;
            }
            if block[1] >= self.height {
                self.active_tetrmomino.y -= 1;
                block[1] -= block[1] - self.height;
            } 
            if self.settled_block_at_coord(block[0], block[1]) {
                return;
            }
        }
        
        self.active_blocks = self.active_tetrmomino.blocks();
    }

    pub fn rotate_blocks_cc(&mut self) {
        self.active_tetrmomino.rotate_cc();
        let mut new_blocks = self.active_tetrmomino.blocks();

        for block in new_blocks.iter_mut() {
            if block[0] < 0 {
                self.active_tetrmomino.x += 1;
            } else if block[0] >= self.width {
                self.active_tetrmomino.x -= 1;
            }
            if block[1] >= self.height {
                self.active_tetrmomino.y -= 1;
                block[1] -= block[1] - self.height;
            } 
            if self.settled_block_at_coord(block[0], block[1]) {
                return;
            }
        }
        
        self.active_blocks = self.active_tetrmomino.blocks();
    }

    pub fn settle_blocks(&mut self) {
        for block in &self.active_blocks {
            self.settled_blocks[((self.width * block[1]) + block[0]) as usize] = Some(self.active_tetrmomino.shape);
        }
    }
    
    pub fn edge_blocks_down(&mut self) {
        for block in &mut self.active_blocks.iter_mut() {
            block[1] += 1;
        }
        self.active_tetrmomino.y += 1;
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
        self.active_tetrmomino.x += 1;
    }

    pub fn move_blocks_left(&mut self) {
        for block in &mut self.active_blocks.iter_mut() {
            block[0] -= 1;
        }
        self.active_tetrmomino.x -= 1;
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

    fn block_at_coord(&self, x: i32, y: i32) -> bool {
        for block in self.active_blocks.iter() {
            if block[0] == x && block[1] == y {
                return true;
            }
        }

        self.settled_block_at_coord(x, y)
    }
    
    fn shape_at_coord(&self, x: i32, y: i32) -> TetrominoShape {
        for block in self.active_blocks.iter() {
            if block[0] == x && block[1] == y {
                return self.active_tetrmomino.shape;
            }
        }

        return self.settled_blocks[((self.width * y) + x) as usize].unwrap();
    }

    fn settled_block_at_coord(&self, x: i32, y: i32) -> bool {
        self.settled_blocks[((self.width * y) + x) as usize].is_some()
    }

    fn clear_row(&mut self, y: i32) {
        for x in 0..self.width {
            if self.settled_blocks[((self.width * y) + x) as usize].is_none() {
                return;
            }
        }
        println!("need to clear row");
        for yi in (1..=y).rev() {
            for x in 0..self.width {
                self.settled_blocks[((self.width * yi) + x) as usize] = self.settled_blocks[((self.width * (yi - 1)) + x) as usize];
            }
        }
    }

    pub fn clear_rows(&mut self) {
        for y in 0..self.height {
            self.clear_row(y);
        }
    }

    pub fn draw(&self) -> String {
        let mut board = String::new();
        for y in 0..self.height {
            println!("{}", y);
            for _ in 0..2 {
                for x in 0..self.width {
                    if self.block_at_coord(x, y) {
                        board.push_str(self.shape_at_coord(x, y).color_str());
                        board.push_str("███");
                    } else {
                        board.push_str("   ");
                    }
                }
                board.push_str("\n");
            }
        }
        board
    }

    pub fn get_start_pos(&self) -> (i32, i32) {
        ((self.width / 2 )- 1, 0)
    }
}