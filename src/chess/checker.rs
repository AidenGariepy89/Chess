use std::mem;

use anyhow::{Result, anyhow};
use super::{utils::{Piece, Player, Move, PieceType}, board::{BOARD_LEN, Board}, keeper::Keeper};

pub struct Snapshot {
    spaces: [Piece; BOARD_LEN],
}

impl Snapshot {
    pub fn new(board: &Board) -> Self {
        Snapshot { spaces: board.get_spaces() }
    }

    pub fn snap(board: &Board, m: &Move) -> Self {
        let mut spaces = board.get_spaces();
        spaces[m.to] = spaces[m.from];
        spaces[m.from] = Piece::None;

        return Snapshot { spaces };
    }
}

pub fn is_in_check(snap: &Snapshot, turn: Player) -> bool {
    let keeper = Keeper::new(&snap.spaces);
    let king_position: usize = match turn {
        Player::White => {
            let pieces = keeper.get_white_pieces();
            let index = pieces.iter().position(|&(x, _)| mem::discriminant(&PieceType::King) == mem::discriminant(&x)).unwrap();
            let (_, pos) = pieces[index];
            pos
        },
        Player::Black => {
            let pieces = keeper.get_black_pieces();
            let index = pieces.iter().position(|&(x, _)| mem::discriminant(&PieceType::King) == mem::discriminant(&x)).unwrap();
            let (_, pos) = pieces[index];
            pos
        }
    };

    let opposing_pieces = match turn {
        Player::White => { keeper.get_black_pieces() },
        Player::Black => { keeper.get_white_pieces() },
    };

    for piece in opposing_pieces {
        let m = Move { from: piece.1, to: king_position };
        // m.is_valid_move(&snap.spaces, turn); WORKS NOW!!!
    }

    return false;
}

