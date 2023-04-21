use std::mem;
use super::utils::{PieceType, Piece, Player, Castle};

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

#[derive(Clone, Copy)]
enum CastleState {
    Unable,
    Able(CastleSide),
}

#[derive(Clone, Copy)]
enum CastleSide {
    Both,
    Short,
    Long,
}

pub struct Keeper {
    white_pieces: Vec<(PieceType, usize)>,
    black_pieces: Vec<(PieceType, usize)>,
    white_castling: CastleState,
    black_castling: CastleState,
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

        return Self {
            white_pieces,
            black_pieces,
            white_castling: CastleState::Able(CastleSide::Both),
            black_castling: CastleState::Able(CastleSide::Both),
        };
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

        (self.white_castling, self.black_castling) = determine_castling_state((self.white_castling, self.black_castling), spaces);

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

    /// Returns:
    /// True only if the pieces needed for castling have not yet moved in the current game.
    pub fn can_castle(&self, options: Castle, turn: Player) -> bool {
        match turn {
            Player::White => {
                if let CastleState::Unable = self.white_castling {
                    return false;
                }
                match options {
                    Castle::Short => {
                        if let CastleState::Able(CastleSide::Long) = self.white_castling {
                            return false;
                        }
                    },
                    Castle::Long => {
                        if let CastleState::Able(CastleSide::Short) = self.white_castling {
                            return false;
                        }
                    },
                }
            },
            Player::Black => {
                if let CastleState::Unable = self.black_castling {
                    return false;
                }
                match options {
                    Castle::Short => {
                        if let CastleState::Able(CastleSide::Long) = self.black_castling {
                            return false;
                        }
                    },
                    Castle::Long => {
                        if let CastleState::Able(CastleSide::Short) = self.black_castling {
                            return false;
                        }
                    },
                }
            },
        }
        return true;
    }
}

/// Returns:
/// - White's castling state first,
/// - Black's castling state second.
fn determine_castling_state(previous_state: (CastleState, CastleState), current: &[Piece]) -> (CastleState, CastleState) {
    if let (CastleState::Unable, CastleState::Unable) = previous_state {
        return previous_state;
    }

    let mut white_can_castle_long = false;
    let mut white_can_castle_short = false;
    let mut black_can_castle_long = false;
    let mut black_can_castle_short = false;

    // Black check first
    if let Some(PieceType::King) = current[4].extract_value() {
        if let Some(PieceType::Rook) = current[0].extract_value() {
            match previous_state.1 {
                CastleState::Able(CastleSide::Short) => { },
                CastleState::Able(_) => { black_can_castle_long = true; },
                _ => { },
            }
        }
        if let Some(PieceType::Rook) = current[7].extract_value() {
            match previous_state.1 {
                CastleState::Able(CastleSide::Long) => { },
                CastleState::Able(_) => { black_can_castle_short = true; },
                _ => { },
            }
        }
    }
    // White check
    if let Some(PieceType::King) = current[60].extract_value() {
        if let Some(PieceType::Rook) = current[56].extract_value() {
            match previous_state.0 {
                CastleState::Able(CastleSide::Short) => { },
                CastleState::Able(_) => { white_can_castle_long = true; },
                _ => { },
            }
        }
        if let Some(PieceType::Rook) = current[63].extract_value() {
            match previous_state.1 {
                CastleState::Able(CastleSide::Long) => { },
                CastleState::Able(_) => { white_can_castle_short = true; },
                _ => { },
            }
        }
    }

    let mut black_state = CastleState::Unable;
    let mut white_state = CastleState::Unable;

    if black_can_castle_short {
        black_state = CastleState::Able(CastleSide::Short);
    }
    if black_can_castle_long {
        black_state = CastleState::Able(CastleSide::Long);
    }
    if black_can_castle_short && black_can_castle_long {
        black_state = CastleState::Able(CastleSide::Both);
    }

    if white_can_castle_short {
        white_state = CastleState::Able(CastleSide::Short);
    }
    if white_can_castle_long {
        white_state = CastleState::Able(CastleSide::Long);
    }
    if white_can_castle_short && white_can_castle_long {
        white_state = CastleState::Able(CastleSide::Both);
    }

    return (white_state, black_state);
}

