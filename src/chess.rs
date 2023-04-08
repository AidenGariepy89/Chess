mod board;
pub mod utils;

use self::utils::LoopState;

use self::board::Board;

pub fn run() -> LoopState {
    let board = Board::new();

    board.test();

    return LoopState::Exit;
}

