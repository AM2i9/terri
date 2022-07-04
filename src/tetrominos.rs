use rand::{distributions::{Distribution, Standard}, Rng,};

#[derive(Debug)]
pub enum RotationState {
    A,
    B,
    C,
    D
}

#[derive(Clone, Copy, Debug)]
pub enum TetrominoShape {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl Distribution<TetrominoShape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetrominoShape {
        match rng.gen_range(0..6) {
            0 => TetrominoShape::I,
            1 => TetrominoShape::O,
            2 => TetrominoShape::T,
            3 => TetrominoShape::S,
            4 => TetrominoShape::Z,
            5 => TetrominoShape::J,
            _ => TetrominoShape::L,
        }
    }
}

#[derive(Debug)]
pub struct Tetromino {
    pub shape: TetrominoShape,
    pub rotation_state: RotationState,
    pub x: i32,
    pub y: i32,
}


impl Tetromino {
    pub fn with_shape(shape: TetrominoShape) -> Tetromino {
        Tetromino {
            shape: shape,
            rotation_state: RotationState::A,
            x: 0,
            y: 0,
        }
    }

    pub fn random() -> Tetromino {
        Tetromino {
            shape: rand::random::<TetrominoShape>(),
            rotation_state: RotationState::A,
            x: 0,
            y: 0,
        }
    }

    pub fn rotate_cw(&mut self) {
        match self.rotation_state {
            RotationState::A => self.rotation_state = RotationState::B,
            RotationState::B => self.rotation_state = RotationState::C,
            RotationState::C => self.rotation_state = RotationState::D,
            RotationState::D => self.rotation_state = RotationState::A,
        }
    }

    pub fn rotate_cc(&mut self) {
        match self.rotation_state {
            RotationState::A => self.rotation_state = RotationState::D,
            RotationState::B => self.rotation_state = RotationState::A,
            RotationState::C => self.rotation_state = RotationState::B,
            RotationState::D => self.rotation_state = RotationState::C,
        }
    }

    pub fn blocks(&self) -> [[i32; 2]; 4] {
        let mut new_blocks = match self.shape {
            TetrominoShape::I => {
                match self.rotation_state {
                    RotationState::A => [[1, 0], [1, 1], [1, 2], [1, 3]],
                    RotationState::B => [[0, 1], [1, 1], [2, 1], [3, 1]],
                    RotationState::C => [[2, 0], [2, 1], [2, 2], [2, 3]],
                    RotationState::D => [[0, 2], [1, 2], [2, 2], [3, 2]],
                }
            },
            TetrominoShape::O => [[0, 0], [0, 1], [1, 0], [1, 1]],
            TetrominoShape::T => {
                match self.rotation_state {
                    RotationState::A => [[1, 0], [1, 1], [0, 1], [2, 1]],
                    RotationState::B => [[1, 0], [1, 1], [1, 2], [2, 1]],
                    RotationState::C => [[0, 1], [1, 1], [1, 2], [2, 1]],
                    RotationState::D => [[0, 1], [1, 1], [1, 2], [1, 0]],
                }
            },
            TetrominoShape::S => {
                match self.rotation_state {
                    RotationState::A | RotationState::C => [[0, 1], [1, 1], [1, 0], [2, 0]],
                    RotationState::B | RotationState::D => [[1, 0], [1, 1], [2, 1], [2, 2]],
                }
            },
            TetrominoShape::Z => {
                match self.rotation_state {
                    RotationState::A | RotationState::C => [[0, 0], [1, 1], [1, 0],[2, 1]],
                    RotationState::B | RotationState::D => [[1, 0], [1, 1], [0, 1], [0, 2]],
                }
            },
            TetrominoShape::J => {
                match self.rotation_state {
                    RotationState::A => [[1, 0], [1, 1], [1, 2], [0, 2]],
                    RotationState::B => [[0, 0], [0, 1], [1, 1], [2, 1]],
                    RotationState::C => [[1, 0], [1, 1], [1, 2], [2, 0]],
                    RotationState::D => [[2, 2], [0, 1], [1, 1], [2, 1]],
                }
            },
            TetrominoShape::L => {
                match self.rotation_state {
                    RotationState::A => [[1, 0], [1, 1], [1, 2], [2, 2]],
                    RotationState::B => [[0, 2], [0, 1], [1, 1], [2, 1]],
                    RotationState::C => [[1, 0], [1, 1], [1, 2], [0, 0]],
                    RotationState::D => [[2, 0], [0, 1], [1, 1], [2, 1]],
                }
            },
        };
        for block in new_blocks.iter_mut() {
            block[0] += self.x;
            block[1] += self.y;
        }
        new_blocks
    }
}
