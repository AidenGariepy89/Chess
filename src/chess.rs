pub mod board;
pub mod utils;
pub mod movement;
pub mod checker;
pub mod keeper;
pub mod interpret;

use colored::*;
use self::board::Board;
use self::utils::Player;
use crate::chess::checker::Snapshot;
use crate::chess::movement::castle;
use crate::input::get_input;

pub enum LoopState {
    Continue,
    Exit,
}

pub fn run(board: &mut Board) -> LoopState {
    clearscr!();

    board.print();

    let snap = Snapshot::new(board);
    if checker::is_in_check(&snap, board.get_turn()) {
        println!("C H E C K !");
    }

    match board.get_turn() {
        Player::White => println!("{}, make your move!", "White".bold()),
        Player::Black => println!("{}, make your move!", "Black".green().bold()),
    }

    let input = get_input().to_lowercase();
    let input = input.trim();

    if input == "q" { return LoopState::Exit; }

    match interpret::interpret_notation(input, board) {
        Err(error) => {
            println!("{} Press enter to continue...", error);

            #[allow(unused_variables)]
            let input = get_input();

            return LoopState::Continue;
        },
        Ok(m) => {
            match m.castle {
                None => {
                    if let Err(error) = board.play(m) {
                        println!("{}", error);

                        #[allow(unused_variables)]
                        let input = get_input();

                        return LoopState::Continue;
                    };
                },
                Some(castle_options) => {
                    if let Err(error) = castle(board, castle_options, board.get_turn()) {
                        println!("{}", error);

                        #[allow(unused_variables)]
                        let input = get_input();

                        return LoopState::Continue;
                    }
                }
            }
        }
    }

    let snap = Snapshot::new(board);
    let promotable_pawns = checker::pawns_to_promote(&snap);
    if !promotable_pawns.is_empty() {
        for pawn in promotable_pawns {
            loop {
                println!("Pawn at {} is able to be promoted! Choose your promotion: Q, B, N, R", pawn);
                let input = get_input().to_lowercase();
                let input = input.trim();
                let player = board.get_space(pawn).unwrap().extract_player().unwrap();
                match input {
                    "q" => {
                        board.change_piece(pawn, utils::Piece::Piece(utils::PlayerPiece { piece: utils::PieceType::Queen, player  }));
                        break;
                    },
                    "b" => {
                        board.change_piece(pawn, utils::Piece::Piece(utils::PlayerPiece { piece: utils::PieceType::Bishop, player  }));
                        break;
                    },
                    "n" => {
                        board.change_piece(pawn, utils::Piece::Piece(utils::PlayerPiece { piece: utils::PieceType::Knight, player  }));
                        break;
                    },
                    "r" => {
                        board.change_piece(pawn, utils::Piece::Piece(utils::PlayerPiece { piece: utils::PieceType::Rook, player  }));
                        break;
                    },
                    _ => { },
                }
            }
        }
    }

    board.next_turn();

    LoopState::Continue
}

