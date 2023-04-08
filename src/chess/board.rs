use super::utils::{Piece, Player};

const ROW_LEN: usize = 8;
const BOARD_LEN: usize = 64;

pub struct Board {
    spaces: [Piece; BOARD_LEN],
    turn: Player,
}

impl Board {
    pub fn new() -> Self {
        Self {
            spaces: [Piece::None; BOARD_LEN],
            turn: Player::White,
        }
    }

    pub fn test(&self) {
        println!("Tesing testing 1 2 3!");
    }
}

