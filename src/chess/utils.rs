// Type Definitions

use colored::*;

#[derive(Clone, Copy)]
pub enum Piece {
    None,
    Piece(PlayerPiece),
}

#[derive(Clone, Copy)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub enum Player {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub struct PlayerPiece {
    pub piece: PieceType,
    pub player: Player,
}

#[derive(Clone, Copy)]
pub struct Move {
    pub from: usize,
    pub to: usize,
}

// Type Implementations

impl Piece {
    pub fn to_colored_string(&self, is_last_move: bool) -> ColoredString {
        match self {
            Piece::None => {
                if is_last_move { return "#".yellow(); }
                return " ".normal();
            }
            Piece::Piece(p) => p.to_colored_string(),
        }
    }
}

impl PlayerPiece {
    pub fn new(piece: PieceType, player: Player) -> Self {
        Self { piece, player }
    }

    pub fn to_colored_string(&self) -> ColoredString {
        match self.player {
            Player::White => {
                match self.piece {
                    PieceType::Pawn => "P".bold(),
                    PieceType::Rook => "R".bold(),
                    PieceType::Knight => "N".bold(),
                    PieceType::Bishop => "B".bold(),
                    PieceType::Queen => "Q".bold(),
                    PieceType::King => "K".bold(),
                }
            },
            Player::Black => {
                match self.piece {
                    PieceType::Pawn => "P".green().bold(),
                    PieceType::Rook => "R".green().bold(),
                    PieceType::Knight => "N".green().bold(),
                    PieceType::Bishop => "B".green().bold(),
                    PieceType::Queen => "Q".green().bold(),
                    PieceType::King => "K".green().bold(),
                }
            },
        }
    }
}

