use chess::chess;
use ::chess::chess::{LoopState, board::Board};

fn main() {
    let mut board = Board::new();
    board.play_no_rules(chess::utils::Move { from: 0, to: 28 }).unwrap();
    let mut result = LoopState::Continue;

    while let LoopState::Continue = result {
        result = chess::run(&mut board);
    }
}

