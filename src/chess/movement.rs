use super::{utils::{Move, Piece, PieceType, PlayerPiece, Player}, board::{Board, BOARD_LEN, ROW_LEN}};
use anyhow::{Result, anyhow};

impl Move {
    pub fn is_valid_move(&self, board: &Board) -> Result<()> {
        if self.from >= BOARD_LEN || self.to >= BOARD_LEN { return Err(anyhow!("Index out of bounds!")); }

        match board.get_space(self.from).unwrap() {
            Piece::None => { return Err(anyhow!("No piece there!")); },
            Piece::Piece(piece) => {
                if let Piece::Piece(target_piece) = board.get_space(self.to).unwrap() {
                    if let (Player::White, Player::White) = (piece.player, target_piece.player) {
                        return Err(anyhow!("Cannot capture your own piece!"));
                    }
                    if let (Player::Black, Player::Black) = (piece.player, target_piece.player) {
                        return Err(anyhow!("Cannot capture your own piece!"));
                    }
                }

                if std::mem::discriminant(&piece.player) != std::mem::discriminant(&board.get_turn()) {
                    return Err(anyhow!("That's not your piece!"));
                }

                match piece.piece {
                    PieceType::Pawn => {
                        return pawn_movement(&board, &self, piece);
                    },
                    PieceType::Rook => {
                        return rook_movement(&board, &self, piece);
                    },
                    PieceType::Knight => {
                        return knight_movement(&board, &self, piece);
                    },
                    PieceType::Bishop => {
                        return bishop_movement(&board, &self, piece);
                    },
                    PieceType::Queen => {
                        return queen_movement(&board, &self, piece);
                    },
                    PieceType::King => {
                        return king_movement(&board, &self, piece);
                    },
                }
            },
        }
    }
}

/// Pawn movement, no capturing
fn pawn_movement(board: &Board, m: &Move, p: PlayerPiece) -> Result<()> {
    match p.player {
        Player::White => {
            if m.to > m.from { return Err(anyhow!("Pawn cannot move backwards!")); }

            if let Piece::Piece(_) = board.get_space(m.to).unwrap() {
                if m.to == m.from - ROW_LEN + 1 || m.to == m.from - ROW_LEN - 1 {
                    return Ok(());
                }
                return Err(anyhow!("Pawn can only capture diagonally one space ahead!"));
            }

            if m.from >= 48 && m.from < 56 {
                if m.from - m.to == ROW_LEN || m.from - m.to == ROW_LEN * 2 {
                    return Ok(());
                }
                return Err(anyhow!("Pawn can only move straight forward!"));
            }

            if m.from - m.to == ROW_LEN {
                return Ok(());
            }
            return Err(anyhow!("Pawn can only move straight forward!"));
        },
        Player::Black => {
            if m.to < m.from { return Err(anyhow!("Pawn cannot move backwards!")); }

            if let Piece::Piece(_) = board.get_space(m.to).unwrap() {
                if m.to == m.from + ROW_LEN + 1 || m.to == m.from + ROW_LEN - 1 {
                    return Ok(());
                }
                return Err(anyhow!("Pawn can only capture diagonally one space ahead!"));
            }

            if m.from >= 8 && m.from < 16 {
                if m.to - m.from == ROW_LEN || m.to - m.from == ROW_LEN * 2 {
                    return Ok(());
                }
                return Err(anyhow!("Pawn can only move straight forward!"));
            }

            if m.to - m.from == ROW_LEN {
                return Ok(());
            }
            return Err(anyhow!("Pawn can only move straight forward!"));
        },
    }
}

fn rook_movement(board: &Board, m: &Move, p: PlayerPiece) -> Result<()> {
    if index_in_raycast(&board, m.from, Direction::North, m.to) {
        match raycast(&board, m.from, Direction::North) {
            None => { return Ok(()); },
            Some(index) => {
                if index > m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }
    if index_in_raycast(&board, m.from, Direction::East, m.to) {
        match raycast(&board, m.from, Direction::East) {
            None => { return Ok(()); },
            Some(index) => {
                if index < m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }
    if index_in_raycast(&board, m.from, Direction::South, m.to) {
        match raycast(&board, m.from, Direction::South) {
            None => { return Ok(()); },
            Some(index) => {
                if index < m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }
    if index_in_raycast(&board, m.from, Direction::West, m.to) {
        match raycast(&board, m.from, Direction::West) {
            None => { return Ok(()); },
            Some(index) => {
                if index > m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }

    return Err(anyhow!("Rooks only move horizontally or vertically!"));
}

fn bishop_movement(board: &Board, m: &Move, p: PlayerPiece) -> Result<()> {
    if index_in_raycast(&board, m.from, Direction::Northeast, m.to) {
        match raycast(&board, m.from, Direction::Northeast) {
            None => { return Ok(()); },
            Some(index) => {
                if index > m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }
    if index_in_raycast(&board, m.from, Direction::Northwest, m.to) {
        match raycast(&board, m.from, Direction::Northwest) {
            None => { return Ok(()); },
            Some(index) => {
                if index > m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }
    if index_in_raycast(&board, m.from, Direction::Southeast, m.to) {
        match raycast(&board, m.from, Direction::Southeast) {
            None => { return Ok(()); },
            Some(index) => {
                if index < m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }
    if index_in_raycast(&board, m.from, Direction::Southwest, m.to) {
        match raycast(&board, m.from, Direction::Southwest) {
            None => { return Ok(()); },
            Some(index) => {
                if index < m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }

    return Err(anyhow!("Bishops only move diagonally!"));
}

fn knight_movement(board: &Board, m: &Move, p: PlayerPiece) -> Result<()> {
    return Err(anyhow!("Not implemented yet!"));
}

fn queen_movement(board: &Board, m: &Move, p: PlayerPiece) -> Result<()> {
    return Err(anyhow!("Not implemented yet!"));
}

fn king_movement(board: &Board, m: &Move, p: PlayerPiece) -> Result<()> {
    return Err(anyhow!("Not implemented yet!"));
}

enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

fn index_in_raycast(board: &Board, from: usize, direction: Direction, needle: usize) -> bool {
    if from >= BOARD_LEN { return false; }

    let mut focus = from;

    match direction {
        Direction::North => {
            while focus >= ROW_LEN {
                focus -= ROW_LEN;

                if focus == needle { return true; }
            }
        },
        Direction::Northeast => {
            while focus >= ROW_LEN && (focus % ROW_LEN) < (ROW_LEN - 1) {
                focus -= ROW_LEN;
                focus += 1;

                if focus == needle { return true; }
            }
        },
        Direction::East => {
            while (focus % ROW_LEN) < (ROW_LEN - 1) {
                focus += 1;

                if focus == needle { return true; }
            }
        },
        Direction::Southeast => {
            while focus < BOARD_LEN - ROW_LEN && (focus % ROW_LEN) < (ROW_LEN - 1) {
                focus += ROW_LEN;
                focus += 1;

                if focus == needle { return true; }
            }
        },
        Direction::South => {
            while focus < BOARD_LEN - ROW_LEN {
                focus += ROW_LEN;

                if focus == needle { return true; }
            }
        },
        Direction::Southwest => {
            while focus < BOARD_LEN - ROW_LEN && (focus % ROW_LEN) > 0 {
                focus += ROW_LEN;
                focus -= 1;

                if focus == needle { return true; }
            }
        },
        Direction::West => {
            while (focus % ROW_LEN) > 0 {
                focus -= 1;

                if focus == needle { return true; }
            }
        },
        Direction::Northwest => {
            while focus >= ROW_LEN && (focus % ROW_LEN) > 0 {
                focus -= ROW_LEN;
                focus -= 1;

                if focus == needle { return true; }
            }
        }
    }

    return false;
}

fn raycast(board: &Board, from: usize, direction: Direction) -> Option<usize> {
    if from >= BOARD_LEN { return None; }

    let mut focus = from;

    match direction {
        Direction::North => {
            while focus >= ROW_LEN {
                focus -= ROW_LEN;

                if let Some(Piece::Piece(_)) = board.get_space(focus) {
                    return Some(focus);
                }
            }
        },
        Direction::Northeast => {
            while focus >= ROW_LEN && (focus % ROW_LEN) < (ROW_LEN - 1) {
                focus -= ROW_LEN;
                focus += 1;

                if let Some(Piece::Piece(_)) = board.get_space(focus) {
                    return Some(focus);
                }
            }
        },
        Direction::East => {
            while (focus % ROW_LEN) < (ROW_LEN - 1) {
                focus += 1;

                if let Some(Piece::Piece(_)) = board.get_space(focus) {
                    return Some(focus);
                }
            }
        },
        Direction::Southeast => {
            while focus < BOARD_LEN - ROW_LEN && (focus % ROW_LEN) < (ROW_LEN - 1) {
                focus += ROW_LEN;
                focus += 1;

                if let Some(Piece::Piece(_)) = board.get_space(focus) {
                    return Some(focus);
                }
            }
        },
        Direction::South => {
            while focus < BOARD_LEN - ROW_LEN {
                focus += ROW_LEN;

                if let Some(Piece::Piece(_)) = board.get_space(focus) {
                    return Some(focus);
                }
            }
        },
        Direction::Southwest => {
            while focus < BOARD_LEN - ROW_LEN && (focus % ROW_LEN) > 0 {
                focus += ROW_LEN;
                focus -= 1;

                if let Some(Piece::Piece(_)) = board.get_space(focus) {
                    return Some(focus);
                }
            }
        },
        Direction::West => {
            while (focus % ROW_LEN) > 0 {
                focus -= 1;

                if let Some(Piece::Piece(_)) = board.get_space(focus) {
                    return Some(focus);
                }
            }
        },
        Direction::Northwest => {
            while focus >= ROW_LEN && (focus % ROW_LEN) > 0 {
                focus -= ROW_LEN;
                focus -= 1;

                if let Some(Piece::Piece(_)) = board.get_space(focus) {
                    return Some(focus);
                }
            }
        }
    }

    return None;
}

#[test]
fn test_raycast() {
    let board = Board::new();

    assert_eq!(raycast(&board, 8, Direction::North), Some(0));
    assert_eq!(raycast(&board, 0, Direction::North), None);
    assert_eq!(raycast(&board, 48, Direction::South), Some(56));
    assert_eq!(raycast(&board, 56, Direction::South), None);
    assert_eq!(raycast(&board, 8, Direction::East), Some(9));
    assert_eq!(raycast(&board, 16, Direction::East), None);
    assert_eq!(raycast(&board, 9, Direction::West), Some(8));
    assert_eq!(raycast(&board, 8, Direction::West), None);
    assert_eq!(raycast(&board, 8, Direction::Northeast), Some(1));
    assert_eq!(raycast(&board, 37, Direction::Northeast), None);
    assert_eq!(raycast(&board, 9, Direction::Northwest), Some(0));
    assert_eq!(raycast(&board, 34, Direction::Northwest), None);
    assert_eq!(raycast(&board, 0, Direction::Southeast), Some(9));
    assert_eq!(raycast(&board, 21, Direction::Southeast), None);
    assert_eq!(raycast(&board, 1, Direction::Southwest), Some(8));
    assert_eq!(raycast(&board, 18, Direction::Southwest), None);
}

