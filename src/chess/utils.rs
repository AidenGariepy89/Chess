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
    piece: PieceType,
    player: Player,
}

pub enum LoopState {
    Continue,
    Exit,
}
