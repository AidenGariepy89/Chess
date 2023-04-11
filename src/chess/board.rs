use super::{
    keeper::Keeper,
    utils::{Move, Piece, PieceType, Player, PlayerPiece},
};
use anyhow::{anyhow, Result};
use colored::*;

pub const ROW_LEN: usize = 8;
pub const BOARD_LEN: usize = 64;

pub struct Board {
    spaces: [Piece; BOARD_LEN],
    turn: Player,
    last_move: Option<Move>,
    keeper: Keeper,
}

impl Board {
    pub fn new() -> Self {
        let spaces = [
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
        ];

        return Self {
            turn: Player::White,
            last_move: None,
            keeper: Keeper::new(&spaces),
            spaces,
        };
    }

    pub fn play(&mut self, m: Move) -> Result<()> {
        m.is_valid_move(&self)?;

        let piece = self.spaces[m.from];

        self.spaces[m.from] = Piece::None;
        self.spaces[m.to] = piece;

        self.last_move = Some(m);

        self.keeper.update(&self.spaces);

        return Ok(());
    }

    pub fn play_no_rules(&mut self, m: Move) -> Result<()> {
        match self.spaces[m.from] {
            Piece::None => {
                return Err(anyhow!("No piece there!"));
            }
            Piece::Piece(p) => {
                self.spaces[m.from] = Piece::None;
                match self.spaces[m.to] {
                    Piece::None => {
                        self.spaces[m.to] = Piece::Piece(p);
                    }
                    Piece::Piece(_) => {
                        return Err(anyhow!("A piece is already there!"));
                    }
                }
            }
        }

        return Ok(());
    }

    pub fn get_space(&self, index: usize) -> Option<Piece> {
        if index >= BOARD_LEN {
            return None;
        }
        return Some(self.spaces[index]);
    }

    pub fn get_spaces(&self) -> [Piece; BOARD_LEN] {
        return self.spaces;
    }

    pub fn get_turn(&self) -> Player {
        self.turn
    }

    pub fn next_turn(&mut self) {
        match self.turn {
            Player::White => {
                self.turn = Player::Black;
            }
            Player::Black => {
                self.turn = Player::White;
            }
        }
    }

    #[allow(dead_code)]
    pub fn test(&self) {
        println!("{:?}", self.keeper.get_black_pieces());
    }

    pub fn print(&self) {
        print!("{} captured pieces: ", "Black".green());
        for piece in self.keeper.get_black_captured() {
            print!("{} ", PlayerPiece::new(piece, Player::White).to_colored_string(false));
        }
        println!("\n");
        println!(" _____ _____ _____ _____ _____ _____ _____ _____");

        for row in 0..ROW_LEN {
            for i in 0..3 {
                for col in 0..ROW_LEN {
                    if i == 0 {
                        print!("|     ");
                    }
                    if i == 1 {
                        let mut is_last_move = false;
                        let mut is_moved = false;
                        if let Some(m) = self.last_move {
                            if m.from == (row * 8) + col {
                                is_last_move = true;
                            }
                            if m.to == (row * 8) + col {
                                is_moved = true;
                            }
                        }

                        if col == 0 {
                            print!( "{}  {}  ", 8 - row, self.spaces[(row * 8) + col] .to_colored_string(is_last_move, is_moved));
                        } else {
                            print!( "|  {}  ", self.spaces[(row * 8) + col] .to_colored_string(is_last_move, is_moved));
                        }
                    }
                    if i == 2 {
                        print!("|_____");
                    }
                }
                println!("|");
            }
        }
        println!("   a     b     c     d     e     f     g     h\n");

        print!("{} captured pieces: ", "White".bold());
        for piece in self.keeper.get_white_captured() {
            print!("{} ", PlayerPiece::new(piece, Player::Black).to_colored_string(false));
        }
        println!("\n");
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
