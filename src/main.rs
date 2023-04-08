use chess::chess;
use ::chess::chess::utils::LoopState;

fn main() {
    let mut result = LoopState::Continue;

    while let LoopState::Continue = result {
        result = chess::run();
    }
}

