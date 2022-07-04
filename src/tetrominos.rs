#[derive(Debug)]
pub struct Block {
    pub x: u32,
    pub y: u32
}

#[derive(Clone, Copy, Debug)]
pub enum Tetromino {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl Tetromino {
    pub fn blocks(&self) -> [[u32; 2]; 4] {
        match *self {
            Tetromino::I => {
                [
                    [0, 0],
                    [0, 1],
                    [0, 2],
                    [0, 3]
                ]
            },
            Tetromino::O => {
                [
                    [0, 0],
                    [0, 1],
                    [1, 0],
                    [1, 1]
                ]
            },
            Tetromino::T => {
                [
                    [1, 0],
                    [1, 1],
                    [0, 1],
                    [2, 1]
                ]
            },
            Tetromino::S => {
                [
                    [0, 1],
                    [1, 1],
                    [1, 0],
                    [2, 0]
                ]
            },
            Tetromino::Z => {
                [
                    [0, 0],
                    [1, 1],
                    [1, 0],
                    [2, 1]
                ]
            },
            Tetromino::J => {
                [
                    [1, 0],
                    [1, 1],
                    [1, 2],
                    [0, 2]
                ]
            },
            Tetromino::L => {
                [
                    [0, 0],
                    [0, 1],
                    [0, 2],
                    [1, 2]
                ]
            },
        }
    }
}
