use super::{utils::{Move, Piece, PieceType, PlayerPiece, Player}, board::{Board, BOARD_LEN, ROW_LEN}};
use anyhow::{Result, anyhow};

impl Move {
    pub fn is_valid_move(&self, board: &Board) -> Result<()> {
        if self.from >= BOARD_LEN || self.to >= BOARD_LEN { return Err(anyhow!("Index out of bounds!")); }
        if self.from == self.to { return Err(anyhow!("You have to actually move a piece!")); }

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

fn king_movement(board: &Board, m: &Move, p: PlayerPiece) -> Result<()> {
    if m.to < m.from { // Moving up or left
        if m.to == m.from - 1 { return Ok(()); } // move just left
        if m.from < ROW_LEN { return Err(anyhow!("The King can only move horizontally, vertically, and diagonally one space!")); } // cant move up on top row
        if m.to == m.from - ROW_LEN || m.to == m.from - ROW_LEN - 1 || m.to == m.from - ROW_LEN + 1 { return Ok(()); } // moving up
    } else { // Moving down or right
        if m.to == m.from + 1 { return Ok(()); } // move just right
        if m.from >= BOARD_LEN - ROW_LEN { return Err(anyhow!("The King can only move horizontally, vertically, and diagonally one space!")); } // cant move down on bottom row
        if m.to == m.from + ROW_LEN || m.to == m.from + ROW_LEN - 1 || m.to == m.from + ROW_LEN + 1 { return Ok(()); } // moving down
    }

    return Err(anyhow!("The King can only move horizontally, vertically, and diagonally one space!"));
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

fn queen_movement(board: &Board, m: &Move, p: PlayerPiece) -> Result<()> {
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

    return Err(anyhow!("The Queen can only move horizontally, vertically, or diagonally!"));
}

fn knight_movement(board: &Board, m: &Move, p: PlayerPiece) -> Result<()> {
    let to = m.to as i32;
    let from = m.from as i32;
    let row_len = ROW_LEN as i32;
    let board_len = BOARD_LEN as i32;

    if to == from - row_len - 2 || to == from - row_len + 2 || to == from + row_len - 2 || to == from + row_len + 2 ||
       to == from - (row_len * 2) - 1 || to == from - (row_len * 2) + 1 || to == from + (row_len * 2) - 1 || to == from + (row_len * 2) + 1 { return Ok(()); }

    return Err(anyhow!("Knights can only move in those weird 'L' shaped patterns!"));
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

