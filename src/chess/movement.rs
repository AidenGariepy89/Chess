use super::{utils::{Move, Piece, PieceType, PlayerPiece, Player, Castle}, board::{BOARD_LEN, ROW_LEN, Board}};
use anyhow::{Result, anyhow};

impl Move {
    pub fn is_valid_move(&self, board: &[Piece; BOARD_LEN], turn: Player) -> Result<()> {
        if self.from >= BOARD_LEN || self.to >= BOARD_LEN { return Err(anyhow!("Index out of bounds!")); }
        if self.from == self.to { return Err(anyhow!("You have to actually move a piece!")); }

        match board[self.from] {
            Piece::None => { Err(anyhow!("No piece there!")) },
            Piece::Piece(piece) => {
                if let Piece::Piece(target_piece) = board[self.to] {
                    if let (Player::White, Player::White) = (piece.player, target_piece.player) {
                        return Err(anyhow!("Cannot capture your own piece!"));
                    }
                    if let (Player::Black, Player::Black) = (piece.player, target_piece.player) {
                        return Err(anyhow!("Cannot capture your own piece!"));
                    }
                }

                if std::mem::discriminant(&piece.player) != std::mem::discriminant(&turn) {
                    return Err(anyhow!("That's not your piece!"));
                }

                match piece.piece {
                    PieceType::Pawn => {
                        pawn_movement(board, self, piece)
                    },
                    PieceType::Rook => {
                        rook_movement(board, self)
                    },
                    PieceType::Knight => {
                        knight_movement(self)
                    },
                    PieceType::Bishop => {
                        bishop_movement(board, self)
                    },
                    PieceType::Queen => {
                        queen_movement(board, self)
                    },
                    PieceType::King => {
                        king_movement(self)
                    },
                }
            },
        }
    }
}

pub fn castle(board: &mut Board, options: Castle, turn: Player) -> Result<()> {
    if board.get_keeper().can_castle(options, turn) {
        match turn {
            Player::Black => {
                match options {
                    Castle::Long => {
                        let space1 = matches!(board.get_space(1), Some(Piece::None));
                        let space2 = matches!(board.get_space(2), Some(Piece::None));
                        let space3 = matches!(board.get_space(3), Some(Piece::None));
                        if space1 && space2 && space3 {
                            board.play_no_rules(Move::new(0, 3)).unwrap();
                            board.play_no_rules(Move::new(4, 2)).unwrap();
                            return Ok(());
                        }
                    },
                    Castle::Short => {
                        let space1 = matches!(board.get_space(5), Some(Piece::None));
                        let space2 = matches!(board.get_space(6), Some(Piece::None));
                        if space1 && space2 {
                            board.play_no_rules(Move::new(7, 5)).unwrap();
                            board.play_no_rules(Move::new(4, 6)).unwrap();
                            return Ok(());
                        }
                    },
                }
            },
            Player::White => {
                match options {
                    Castle::Long => {
                        let space1 = matches!(board.get_space(57), Some(Piece::None));
                        let space2 = matches!(board.get_space(58), Some(Piece::None));
                        let space3 = matches!(board.get_space(59), Some(Piece::None));
                        if space1 && space2 && space3 {
                            board.play_no_rules(Move::new(56, 59)).unwrap();
                            board.play_no_rules(Move::new(60, 58)).unwrap();
                            return Ok(());
                        }
                    },
                    Castle::Short => {
                        let space1 = matches!(board.get_space(61), Some(Piece::None));
                        let space2 = matches!(board.get_space(62), Some(Piece::None));
                        if space1 && space2 {
                            board.play_no_rules(Move::new(63, 61)).unwrap();
                            board.play_no_rules(Move::new(60, 62)).unwrap();
                            return Ok(());
                        }
                    },
                }
            },
        }

        return Err(anyhow!("There are pieces in the way!"));
    }

    Err(anyhow!("Castling is currently not valid!"))
}

fn pawn_movement(board: &[Piece], m: &Move, p: PlayerPiece) -> Result<()> {
    match p.player {
        Player::White => {
            if m.to > m.from { return Err(anyhow!("Pawn cannot move backwards!")); }
            if m.from < ROW_LEN { return Err(anyhow!("This message should never be read, if the pawn is on top row should be promoted")); }

            if let Piece::Piece(_) = board[m.to] {
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
            Err(anyhow!("Pawn can only move straight forward!"))
        },
        Player::Black => {
            if m.to < m.from { return Err(anyhow!("Pawn cannot move backwards!")); }

            if let Piece::Piece(_) = board[m.to] {
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
            Err(anyhow!("Pawn can only move straight forward!"))
        },
    }
}

fn king_movement(m: &Move) -> Result<()> {
    if m.to < m.from { // Moving up or left
        if m.to == m.from - 1 { return Ok(()); } // move just left
        if m.from < ROW_LEN { return Err(anyhow!("The King can only move horizontally, vertically, and diagonally one space!")); } // cant move up on top row
        if m.to == m.from - ROW_LEN || m.to == m.from - ROW_LEN - 1 || m.to == m.from - ROW_LEN + 1 { return Ok(()); } // moving up
    } else { // Moving down or right
        if m.to == m.from + 1 { return Ok(()); } // move just right
        if m.from >= BOARD_LEN - ROW_LEN { return Err(anyhow!("The King can only move horizontally, vertically, and diagonally one space!")); } // cant move down on bottom row
        if m.to == m.from + ROW_LEN || m.to == m.from + ROW_LEN - 1 || m.to == m.from + ROW_LEN + 1 { return Ok(()); } // moving down
    }

    Err(anyhow!("The King can only move horizontally, vertically, and diagonally one space!"))
}

fn rook_movement(board: &[Piece], m: &Move) -> Result<()> {
    if index_in_raycast(m.from, Direction::North, m.to) {
        match raycast(board, m.from, Direction::North) {
            None => { return Ok(()); },
            Some(index) => {
                if index > m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }
    if index_in_raycast(m.from, Direction::East, m.to) {
        match raycast(board, m.from, Direction::East) {
            None => { return Ok(()); },
            Some(index) => {
                if index < m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }
    if index_in_raycast(m.from, Direction::South, m.to) {
        match raycast(board, m.from, Direction::South) {
            None => { return Ok(()); },
            Some(index) => {
                if index < m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }
    if index_in_raycast(m.from, Direction::West, m.to) {
        match raycast(board, m.from, Direction::West) {
            None => { return Ok(()); },
            Some(index) => {
                if index > m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }

    Err(anyhow!("Rooks only move horizontally or vertically!"))
}

fn bishop_movement(board: &[Piece], m: &Move) -> Result<()> {
    if index_in_raycast(m.from, Direction::Northeast, m.to) {
        match raycast(board, m.from, Direction::Northeast) {
            None => { return Ok(()); },
            Some(index) => {
                if index > m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }
    if index_in_raycast(m.from, Direction::Northwest, m.to) {
        match raycast(board, m.from, Direction::Northwest) {
            None => { return Ok(()); },
            Some(index) => {
                if index > m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }
    if index_in_raycast(m.from, Direction::Southeast, m.to) {
        match raycast(board, m.from, Direction::Southeast) {
            None => { return Ok(()); },
            Some(index) => {
                if index < m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }
    if index_in_raycast(m.from, Direction::Southwest, m.to) {
        match raycast(board, m.from, Direction::Southwest) {
            None => { return Ok(()); },
            Some(index) => {
                if index < m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }

    Err(anyhow!("Bishops only move diagonally!"))
}

fn queen_movement(board: &[Piece], m: &Move) -> Result<()> {
    if index_in_raycast(m.from, Direction::North, m.to) {
        match raycast(board, m.from, Direction::North) {
            None => { return Ok(()); },
            Some(index) => {
                if index > m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }
    if index_in_raycast(m.from, Direction::East, m.to) {
        match raycast(board, m.from, Direction::East) {
            None => { return Ok(()); },
            Some(index) => {
                if index < m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }
    if index_in_raycast(m.from, Direction::South, m.to) {
        match raycast(board, m.from, Direction::South) {
            None => { return Ok(()); },
            Some(index) => {
                if index < m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }
    if index_in_raycast(m.from, Direction::West, m.to) {
        match raycast(board, m.from, Direction::West) {
            None => { return Ok(()); },
            Some(index) => {
                if index > m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }
    if index_in_raycast(m.from, Direction::Northeast, m.to) {
        match raycast(board, m.from, Direction::Northeast) {
            None => { return Ok(()); },
            Some(index) => {
                if index > m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }
    if index_in_raycast(m.from, Direction::Northwest, m.to) {
        match raycast(board, m.from, Direction::Northwest) {
            None => { return Ok(()); },
            Some(index) => {
                if index > m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }
    if index_in_raycast(m.from, Direction::Southeast, m.to) {
        match raycast(board, m.from, Direction::Southeast) {
            None => { return Ok(()); },
            Some(index) => {
                if index < m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }
    if index_in_raycast(m.from, Direction::Southwest, m.to) {
        match raycast(board, m.from, Direction::Southwest) {
            None => { return Ok(()); },
            Some(index) => {
                if index < m.to { return Err(anyhow!("There is a piece in the way!")); }
                return Ok(());
            }
        }
    }

    Err(anyhow!("The Queen can only move horizontally, vertically, or diagonally!"))
}

fn knight_movement(m: &Move) -> Result<()> {
    let to = m.to as i32;
    let from = m.from as i32;
    let row_len = ROW_LEN as i32;

    if to == from - row_len - 2 || to == from - row_len + 2 || to == from + row_len - 2 || to == from + row_len + 2 ||
       to == from - (row_len * 2) - 1 || to == from - (row_len * 2) + 1 || to == from + (row_len * 2) - 1 || to == from + (row_len * 2) + 1 { return Ok(()); }

    Err(anyhow!("Knights can only move in those weird 'L' shaped patterns!"))
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

fn index_in_raycast(from: usize, direction: Direction, needle: usize) -> bool {
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

    false
}

fn raycast(board: &[Piece], from: usize, direction: Direction) -> Option<usize> {
    if from >= BOARD_LEN { return None; }

    let mut focus = from;

    match direction {
        Direction::North => {
            while focus >= ROW_LEN {
                focus -= ROW_LEN;

                if let Piece::Piece(_) = board[focus] {
                    return Some(focus);
                }
            }
        },
        Direction::Northeast => {
            while focus >= ROW_LEN && (focus % ROW_LEN) < (ROW_LEN - 1) {
                focus -= ROW_LEN;
                focus += 1;

                if let Piece::Piece(_) = board[focus] {
                    return Some(focus);
                }
            }
        },
        Direction::East => {
            while (focus % ROW_LEN) < (ROW_LEN - 1) {
                focus += 1;

                if let Piece::Piece(_) = board[focus] {
                    return Some(focus);
                }
            }
        },
        Direction::Southeast => {
            while focus < BOARD_LEN - ROW_LEN && (focus % ROW_LEN) < (ROW_LEN - 1) {
                focus += ROW_LEN;
                focus += 1;

                if let Piece::Piece(_) = board[focus] {
                    return Some(focus);
                }
            }
        },
        Direction::South => {
            while focus < BOARD_LEN - ROW_LEN {
                focus += ROW_LEN;

                if let Piece::Piece(_) = board[focus] {
                    return Some(focus);
                }
            }
        },
        Direction::Southwest => {
            while focus < BOARD_LEN - ROW_LEN && (focus % ROW_LEN) > 0 {
                focus += ROW_LEN;
                focus -= 1;

                if let Piece::Piece(_) = board[focus] {
                    return Some(focus);
                }
            }
        },
        Direction::West => {
            while (focus % ROW_LEN) > 0 {
                focus -= 1;

                if let Piece::Piece(_) = board[focus] {
                    return Some(focus);
                }
            }
        },
        Direction::Northwest => {
            while focus >= ROW_LEN && (focus % ROW_LEN) > 0 {
                focus -= ROW_LEN;
                focus -= 1;

                if let Piece::Piece(_) = board[focus] {
                    return Some(focus);
                }
            }
        }
    }

    None
}

