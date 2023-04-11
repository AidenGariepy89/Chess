use std::mem;

use super::{utils::{PieceType, Piece, Player}, board::Board};

const FULL_SET: [PieceType; 16] = [
    PieceType::Pawn, PieceType::Pawn,
    PieceType::Pawn, PieceType::Pawn,
    PieceType::Pawn, PieceType::Pawn,
    PieceType::Pawn, PieceType::Pawn,
    PieceType::Rook, PieceType::Rook,
    PieceType::Knight, PieceType::Knight,
    PieceType::Bishop, PieceType::Bishop,
    PieceType::Queen, PieceType::King,
];

pub struct Keeper {
    white_pieces: Vec<(PieceType, usize)>,
    black_pieces: Vec<(PieceType, usize)>,
}

impl Keeper {
    pub fn new(spaces: &[Piece]) -> Self {
        let mut white_pieces: Vec<(PieceType, usize)> = Vec::new();
        let mut black_pieces: Vec<(PieceType, usize)> = Vec::new();

        for i in 0..spaces.len() {
            if let Piece::Piece(p) = spaces[i] {
                match p.player {
                    Player::White => { white_pieces.push((p.piece, i)); },
                    Player::Black => { black_pieces.push((p.piece, i)); }
                }
            }
        }

        return Self { white_pieces, black_pieces };
    }

    pub fn update(&mut self, spaces: &[Piece]) {
        let mut white_pieces: Vec<(PieceType, usize)> = Vec::new();
        let mut black_pieces: Vec<(PieceType, usize)> = Vec::new();

        for i in 0..spaces.len() {
            if let Piece::Piece(p) = spaces[i] {
                match p.player {
                    Player::White => { white_pieces.push((p.piece, i)); },
                    Player::Black => { black_pieces.push((p.piece, i)); }
                }
            }
        }

        self.white_pieces = white_pieces;
        self.black_pieces = black_pieces;
    }

    pub fn get_black_captured(&self) -> Vec<PieceType> {
        let mut pieces: Vec<PieceType> = FULL_SET.to_vec();
        for (p, _) in &self.white_pieces {
            if let Some(index) = pieces.iter().position(|&x| mem::discriminant(&x) == mem::discriminant(p)) {
                pieces.remove(index);
            }
        }
        return pieces;
    }

    pub fn get_white_captured(&self) -> Vec<PieceType> {
        let mut pieces: Vec<PieceType> = FULL_SET.to_vec();
        for (p, _) in &self.black_pieces {
            if let Some(index) = pieces.iter().position(|&x| mem::discriminant(&x) == mem::discriminant(p)) {
                pieces.remove(index);
            }
        }
        return pieces;
    }

    pub fn get_black_pieces(&self) -> Vec<(PieceType, usize)> { self.black_pieces.clone() }

    pub fn get_white_pieces(&self) -> Vec<(PieceType, usize)> { self.white_pieces.clone() }
}

