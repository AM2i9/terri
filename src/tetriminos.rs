#[derive(Debug)]
pub struct Block {
    pub x: u32,
    pub y: u32
}

pub enum TetriminoShapes {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl TetriminoShapes {
    pub fn blocks(&self) -> [Block; 4] {
        match *self {
            TetriminoShapes::I => {
                [
                    Block { x: 0, y: 0 },
                    Block { x: 0, y: 1 },
                    Block { x: 0, y: 2 },
                    Block { x: 0, y: 3 }
                ]
            },
            TetriminoShapes::O => {
                [
                    Block { x: 0, y: 0 }, Block { x: 1, y: 0 },
                    Block { x: 0, y: 1 }, Block { x: 1, y: 1 }
                ]
            },
            TetriminoShapes::T => {
                [
                    Block { x: 1, y: 0 },
                    Block { x: 1, y: 1 },
                    Block { x: 0, y: 1 },
                    Block { x: 2, y: 1 }
                ]
            },
            TetriminoShapes::S => {
                [
                    Block { x: 0, y: 1 },
                    Block { x: 1, y: 1 },
                    Block { x: 1, y: 0 },
                    Block { x: 2, y: 0 }
                ]
            },
            TetriminoShapes::Z => {
                [
                    Block { x: 0, y: 0 },
                    Block { x: 1, y: 1 },
                    Block { x: 1, y: 0 },
                    Block { x: 2, y: 1 }
                ]
            },
            TetriminoShapes::J => {
                [
                    Block { x: 1, y: 0 },
                    Block { x: 1, y: 1 },
                    Block { x: 1, y: 2 },
                    Block { x: 0, y: 2 }
                ]
            },
            TetriminoShapes::L => {
                [
                    Block { x: 0, y: 0 },
                    Block { x: 0, y: 1 },
                    Block { x: 0, y: 2 },
                    Block { x: 1, y: 2 }
                ]
            },
        }
    }
}

struct Tetrmimino {}

