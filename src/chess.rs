pub mod board;
pub mod utils;

use anyhow::{Result, anyhow};
use colored::*;
use self::board::{Board, ROW_LEN};
use self::utils::{Player, Move};
use crate::input::get_input;

pub enum LoopState {
    Continue,
    Exit,
}

pub fn run(board: &mut Board) -> LoopState {
    clearscr!();

    board.print();

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
            board.move_no_rules(m).unwrap();
        }
    }

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
        return Ok(Move { from: indices[0], to: indices[1] });
    }
    
    return Err(anyhow!("Not finished yet!"));
}

