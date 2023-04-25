// Type Definitions

use colored::*;

#[derive(Clone, Copy)]
pub enum Piece {
    None,
    Piece(PlayerPiece),
}

#[derive(Debug, Clone, Copy)]
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
    pub castle: Option<Castle>,
}

#[derive(Clone, Copy)]
pub enum Castle {
    Short,
    Long,
}

// Type Implementations

impl Move {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to, castle: None }
    }
    pub fn castle(castle_long: bool) -> Self {
        let castle = if castle_long { Castle::Long } else { Castle::Short };
        Self { from: 0, to: 0, castle: Some(castle) }
    }
}

impl Piece {
    pub fn extract_value(&self) -> Option<PieceType> {
        if let Piece::Piece(p) = self {
            return Some(p.piece);
        }
        None
    }
    pub fn extract_player(&self) -> Option<Player> {
        if let Piece::Piece(p) = self {
            return Some(p.player);
        }
        None
    }
    pub fn to_colored_string(&self, is_last_move: bool, is_moved: bool) -> ColoredString {
        match self {
            Piece::None => {
                if is_last_move { return "#".truecolor(138, 138, 138); }

                " ".normal()
            }
            Piece::Piece(p) => p.to_colored_string(is_moved),
        }
    }
}

impl PlayerPiece {
    pub fn new(piece: PieceType, player: Player) -> Self {
        Self { piece, player }
    }

    pub fn to_colored_string(&self, is_moved: bool) -> ColoredString {
        #[allow(unused_assignments)]
        let mut output = "".normal();
        match self.player {
            Player::White => {
                match self.piece {
                    PieceType::Pawn => { output = "P".bold() },
                    PieceType::Rook => { output = "R".bold() },
                    PieceType::Knight => { output = "N".bold() },
                    PieceType::Bishop => { output = "B".bold() },
                    PieceType::Queen => { output = "Q".bold() },
                    PieceType::King => { output = "K".bold() },
                }
            },
            Player::Black => {
                match self.piece {
                    PieceType::Pawn => { output = "P".green().bold() },
                    PieceType::Rook => { output = "R".green().bold() },
                    PieceType::Knight => { output = "N".green().bold() },
                    PieceType::Bishop => { output = "B".green().bold() },
                    PieceType::Queen => { output = "Q".green().bold() },
                    PieceType::King => { output = "K".green().bold() },
                }
            },
        }
        if is_moved {
            output = output.on_truecolor(92, 91, 91);
        }

        output
    }
}

