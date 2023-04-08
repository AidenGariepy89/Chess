mod board;
pub mod utils;

use self::utils::LoopState;
use self::board::Board;

pub fn run() -> LoopState {
    let mut board = Board::new();

    board.print();

    return LoopState::Exit;
}

