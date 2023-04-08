use anyhow::{Result, anyhow};
use super::utils::{Piece, Player, PieceType, PlayerPiece, Move};

pub const ROW_LEN: usize = 8;
pub const BOARD_LEN: usize = 64;

pub struct Board {
    spaces: [Piece; BOARD_LEN],
    turn: Player,
}

impl Board {
    pub fn new() -> Self {
        Self {
            turn: Player::White,
            spaces: [
                Piece::Piece(PlayerPiece::new(PieceType::Rook, Player::Black)),
                Piece::Piece(PlayerPiece::new(PieceType::Knight, Player::Black)),
                Piece::Piece(PlayerPiece::new(PieceType::Bishop, Player::Black)),
                Piece::Piece(PlayerPiece::new(PieceType::Queen, Player::Black)),
                Piece::Piece(PlayerPiece::new(PieceType::King, Player::Black)),
                Piece::Piece(PlayerPiece::new(PieceType::Bishop, Player::Black)),
                Piece::Piece(PlayerPiece::new(PieceType::Knight, Player::Black)),
                Piece::Piece(PlayerPiece::new(PieceType::Rook, Player::Black)),
                Piece::Piece(PlayerPiece::new(PieceType::Pawn, Player::Black)),
                Piece::Piece(PlayerPiece::new(PieceType::Pawn, Player::Black)),
                Piece::Piece(PlayerPiece::new(PieceType::Pawn, Player::Black)),
                Piece::Piece(PlayerPiece::new(PieceType::Pawn, Player::Black)),
                Piece::Piece(PlayerPiece::new(PieceType::Pawn, Player::Black)),
                Piece::Piece(PlayerPiece::new(PieceType::Pawn, Player::Black)),
                Piece::Piece(PlayerPiece::new(PieceType::Pawn, Player::Black)),
                Piece::Piece(PlayerPiece::new(PieceType::Pawn, Player::Black)),
                Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
                Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
                Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
                Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
                Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
                Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
                Piece::None, Piece::None,
                Piece::Piece(PlayerPiece::new(PieceType::Pawn, Player::White)),
                Piece::Piece(PlayerPiece::new(PieceType::Pawn, Player::White)),
                Piece::Piece(PlayerPiece::new(PieceType::Pawn, Player::White)),
                Piece::Piece(PlayerPiece::new(PieceType::Pawn, Player::White)),
                Piece::Piece(PlayerPiece::new(PieceType::Pawn, Player::White)),
                Piece::Piece(PlayerPiece::new(PieceType::Pawn, Player::White)),
                Piece::Piece(PlayerPiece::new(PieceType::Pawn, Player::White)),
                Piece::Piece(PlayerPiece::new(PieceType::Pawn, Player::White)),
                Piece::Piece(PlayerPiece::new(PieceType::Rook, Player::White)),
                Piece::Piece(PlayerPiece::new(PieceType::Knight, Player::White)),
                Piece::Piece(PlayerPiece::new(PieceType::Bishop, Player::White)),
                Piece::Piece(PlayerPiece::new(PieceType::Queen, Player::White)),
                Piece::Piece(PlayerPiece::new(PieceType::King, Player::White)),
                Piece::Piece(PlayerPiece::new(PieceType::Bishop, Player::White)),
                Piece::Piece(PlayerPiece::new(PieceType::Knight, Player::White)),
                Piece::Piece(PlayerPiece::new(PieceType::Rook, Player::White)),
            ],
        }
    }

    pub fn move_no_rules(&mut self, m: Move) -> Result<()> {
        match self.spaces[m.from] {
            Piece::None => { return Err(anyhow!("No piece there!")); },
            Piece::Piece(p) => {
                self.spaces[m.from] = Piece::None;
                match self.spaces[m.to] {
                    Piece::None => {
                        self.spaces[m.to] = Piece::Piece(p);
                    },
                    Piece::Piece(_) => { return Err(anyhow!("A piece is already there!")); },
                }
            },
        }

        return Ok(());
    }

    pub fn get_turn(&self) -> Player { self.turn }

    pub fn next_turn(&mut self) {
        match self.turn {
            Player::White => { self.turn = Player::Black; },
            Player::Black => { self.turn = Player::White; },
        }
    }

    pub fn test(&self) {
        println!("Tesing testing 1 2 3!");
    }

    pub fn print(&self) {
        println!(" _____ _____ _____ _____ _____ _____ _____ _____");

        for row in 0..ROW_LEN {
            for i in 0..3 {
                for col in 0..ROW_LEN {
                    if i == 0 { print!("|     "); }
                    if i == 1 {
                        if col == 0 {
                            print!("{}  {}  ", 8 - row, self.spaces[(row * 8) + col].to_colored_string());
                        } else {
                            print!("|  {}  ", self.spaces[(row * 8) + col].to_colored_string());
                        }
                    }
                    if i == 2 { print!("|_____"); }
                }
                println!("|");
            }
        }
        println!("   a     b     c     d     e     f     g     h");
    }
}


// _____ _____ _____ _____ _____ _____ _____ _____
//|     |     |     |     |     |     |     |     |
//|  R  |  N  |  B  |  K  |  Q  |  B  |  N  |  R  |
//|_____|_____|_____|_____|_____|_____|_____|_____|
//|     |     |     |     |     |     |     |     |
//|  P  |  P  |  P  |  P  |  P  |  P  |  P  |  P  |
//|_____|_____|_____|_____|_____|_____|_____|_____|
//|     |     |     |     |     |     |     |     |
//|     |     |     |     |     |     |     |     |
//|_____|_____|_____|_____|_____|_____|_____|_____|
//|     |     |     |     |     |     |     |     |
//|     |     |     |     |     |     |     |     |
//|_____|_____|_____|_____|_____|_____|_____|_____|
//|     |     |     |     |     |     |     |     |
//|     |     |     |     |     |     |     |     |
//|_____|_____|_____|_____|_____|_____|_____|_____|
//|     |     |     |     |     |     |     |     |
//|     |     |     |     |     |     |     |     |
//|_____|_____|_____|_____|_____|_____|_____|_____|
//|     |     |     |     |     |     |     |     |
//|  P  |  P  |  P  |  P  |  P  |  P  |  P  |  P  |
//|_____|_____|_____|_____|_____|_____|_____|_____|
//|     |     |     |     |     |     |     |     |
//|  R  |  N  |  B  |  K  |  Q  |  B  |  N  |  R  |
//|_____|_____|_____|_____|_____|_____|_____|_____|
//
//
//
//
//
//
//
//
