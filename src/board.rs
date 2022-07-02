pub struct Board {
    pub height: u32,
    pub width: u32
}

impl Board {
    pub fn draw(&self) -> String {
        let mut board = String::new();
        for _ in 0..(self.height * 4) {
            for _ in 0..(self.width * 4) {
                board.push(' ');
            }
            board.push_str("\n");
        }
        board
    }
}