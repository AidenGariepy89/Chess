use chess::chess;
use ::chess::chess::{LoopState, board::Board};

fn main() {
    let mut board = Board::new();
    let mut result = LoopState::Continue;

    while let LoopState::Continue = result {
        result = chess::run(&mut board);
    }
}

