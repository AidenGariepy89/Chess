use chess::chess;
use ::chess::chess::{LoopState, board::Board};

fn main() {
    let mut board = Board::new();
    let mut result = LoopState::Continue;

    // board.play_no_rules(chess::utils::Move::new(1, 16)).unwrap();
    // board.play_no_rules(chess::utils::Move::new(2, 17)).unwrap();
    // board.play_no_rules(chess::utils::Move::new(3, 18)).unwrap();
    // board.play_no_rules(chess::utils::Move::new(5, 19)).unwrap();
    // board.play_no_rules(chess::utils::Move::new(6, 20)).unwrap();

    // board.play_no_rules(chess::utils::Move::new(57, 21)).unwrap();
    // board.play_no_rules(chess::utils::Move::new(58, 22)).unwrap();
    // board.play_no_rules(chess::utils::Move::new(59, 23)).unwrap();
    // board.play_no_rules(chess::utils::Move::new(61, 24)).unwrap();
    // board.play_no_rules(chess::utils::Move::new(62, 25)).unwrap();

    while let LoopState::Continue = result {
        result = chess::run(&mut board);
    }
}

