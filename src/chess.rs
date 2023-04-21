pub mod board;
pub mod utils;
pub mod movement;
pub mod checker;
pub mod keeper;

use anyhow::{Result, anyhow};
use colored::*;
use self::board::{Board, ROW_LEN};
use self::utils::{Player, Move};
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

    let snap = Snapshot::new(&board);
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

    match interpret_notation(input) {
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

    board.next_turn();

    return LoopState::Continue;
}

fn interpret_notation(input: &str) -> Result<Move> {
    let args: Vec<_> = input.split(" ").collect();

    if args.len() > 1 {
        let mut indices: [usize; 2] = [0; 2];
        for i in 0..2 {
            let mut it = args[i].chars();
            let col = it.next().unwrap_or_else(|| 'z');
            let row = it.next().unwrap_or_else(|| '9');

            if col < 'a' { return Err(anyhow!("Invalid input!")); }
            if row > '8' { return Err(anyhow!("Invalid input!")); }

            let col = col as usize - 'a' as usize;
            let row = '8' as usize - row as usize;

            if col >= ROW_LEN { return Err(anyhow!("Index out of range!")); }
            if row >= ROW_LEN { return Err(anyhow!("Index out of range!")); }

            indices[i] = (row * ROW_LEN) + col;
        }
        return Ok(Move::new(indices[0], indices[1]));
    }

    let mut it = args[0].chars();

    if let Some(value) = it.next() {
        match value {
            '0' => {
                let zeroes: Vec<_> = args[0].split('-').collect();
                if zeroes.len() == 2 { return Ok(Move::castle(false)); }
                if zeroes.len() == 3 { return Ok(Move::castle(true)); }
                return Err(anyhow!("Invalid input!"));
            },
            _ => { return Err(anyhow!("Not implemented yet!")); }
        }
    }
    
    return Err(anyhow!("You can't input nothing!"));
}

