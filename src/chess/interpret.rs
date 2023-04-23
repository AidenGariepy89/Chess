use std::mem;

use anyhow::{anyhow, Result};

use super::{utils::{Move, PieceType, Player}, board::{ROW_LEN, Board}};

pub fn interpret_notation(input: &str, board: &Board) -> Result<Move> {
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
            _ => {
                return complex_interpretation(input, board);
            }
        }
    }
    
    return Err(anyhow!("You can't input nothing!"));
}

fn complex_interpretation(input: &str, board: &Board) -> Result<Move> {
    let mut it = input.chars();
    let len = input.len();

    if len == 2 {
        let col = it.next().unwrap_or_else(|| 'z');
        let row = it.next().unwrap_or_else(|| '9');
        if col < 'a' { return Err(anyhow!("Invalid input!")); }
        if row > '8' { return Err(anyhow!("Invalid input!")); }
        let col = col as usize - 'a' as usize;
        let row = '8' as usize - row as usize;
        if col >= ROW_LEN { return Err(anyhow!("Index out of range!")); }
        if row >= ROW_LEN { return Err(anyhow!("Index out of range!")); }
        let target = (row * ROW_LEN) + col;

        let pawns_found = get_pawns_in_column(col, board);

        if pawns_found.len() > 1 {
            let mut acceptable_moves = Vec::<Move>::new();
            for pawn in &pawns_found {
                if let Some(m) = generate_pawn_move(*pawn, target, board.get_turn()) {
                    acceptable_moves.push(m);
                }
            }
            if acceptable_moves.len() > 1 {
                return Err(anyhow!("Could not distinquish between pawns!"));
            }
            if acceptable_moves.len() == 0 {
                return Err(anyhow!("Invalid pawn move!"));
            }
            return Ok(acceptable_moves[0]);
        }
        if pawns_found.len() == 0 { return Err(anyhow!("No pawns in that column!")); }
        let turn = generate_pawn_move(pawns_found[0], target, board.get_turn());
        match turn {
            None => { return Err(anyhow!("Invalid pawn move!")); },
            Some(m) => { return Ok(m); }
        }
    }

    return Err(anyhow!("Not implemented yet! [from complex_interpretation]"));
}

fn get_pawns_in_column(col: usize, board: &Board) -> Vec<usize> {
    assert!(col < ROW_LEN);
    let mut pawns_found: Vec<usize> = Vec::new();
    for i in 0..ROW_LEN {
        let index = (i * ROW_LEN) + col;
        if let Some(piece) = board.get_space(index) {
            if let Some(PieceType::Pawn) = piece.extract_value() {
                if mem::discriminant(&board.get_turn()) == mem::discriminant(&piece.extract_player().unwrap()) {
                    pawns_found.push(index);
                }
            }
        }
    }
    return pawns_found;
}

// No capturing yet
fn generate_pawn_move(from: usize, to: usize, turn: Player) -> Option<Move> {
    match turn {
        Player::White => {
            if to >= from { return None; }
            if from >= 48 && from < 56 { // Pawn in home row
                if to == from - ROW_LEN || to == from - (ROW_LEN * 2) {
                    return Some(Move::new(from, to));
                }
            }
            if to == from - ROW_LEN {
                return Some(Move::new(from, to));
            }
        },
        Player::Black => {
            if to <= from { return None; }
            if from >= 8 && from < 15 { // Pawn in home row
                if to == from + ROW_LEN || to == from + (ROW_LEN * 2) {
                    return Some(Move::new(from, to));
                }
            }
            if to == from + ROW_LEN {
                return Some(Move::new(from, to));
            }
        }
    }
    return None;
}

