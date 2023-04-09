use super::{utils::{Move, Piece, PieceType, PlayerPiece, Player}, board::{Board, BOARD_LEN, ROW_LEN}};

impl Move {
    pub fn is_valid_move(&self, board: &Board) -> bool {
        if self.from >= BOARD_LEN || self.to >= BOARD_LEN { return false; }

        match board.get_space(self.from).unwrap() {
            Piece::None => { return false; },
            Piece::Piece(piece) => {
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
fn pawn_movement(board: &Board, m: &Move, p: PlayerPiece) -> bool {
    match p.player {
        Player::White => {
            if m.to > m.from { return false; }

            if m.from >= 48 && m.from < 56 {
                return m.from - m.to == ROW_LEN || m.from - m.to == ROW_LEN * 2;
            }

            return m.from - m.to == ROW_LEN;
        },
        Player::Black => {
            if m.to < m.from { return false; }

            if m.from >= 8 && m.from < 16 {
                return m.to - m.from == ROW_LEN || m.to - m.from == ROW_LEN * 2;
            }

            return m.to - m.from == ROW_LEN;
        },
    }
}

#[test]
fn pawn_move_test() {
    let mut board = Board::new();

    let m = Move { from: 8, to: 16 };
    assert_eq!(m.is_valid_move(&board), true);
    let m = Move { from: 8, to: 24 };
    assert_eq!(m.is_valid_move(&board), true);
    let m = Move { from: 8, to: 16 };
    board.move_no_rules(m).unwrap();
    let m = Move { from: 16, to: 24 };
    assert_eq!(m.is_valid_move(&board), true);
    let m = Move { from: 16, to: 32 };
    assert_eq!(m.is_valid_move(&board), false);

    let m = Move { from: 48, to: 40 };
    assert_eq!(m.is_valid_move(&board), true);
    let m = Move { from: 48, to: 32 };
    assert_eq!(m.is_valid_move(&board), true);
    let m = Move { from: 48, to: 40 };
    board.move_no_rules(m).unwrap();
    let m = Move { from: 40, to: 24 };
    assert_eq!(m.is_valid_move(&board), false);
    let m = Move { from: 40, to: 32 };
    assert_eq!(m.is_valid_move(&board), true);
}

fn rook_movement(board: &Board, m: &Move, p: PlayerPiece) -> bool {
    return false;
}

fn knight_movement(board: &Board, m: &Move, p: PlayerPiece) -> bool {
    return false;
}

fn bishop_movement(board: &Board, m: &Move, p: PlayerPiece) -> bool {
    return false;
}

fn queen_movement(board: &Board, m: &Move, p: PlayerPiece) -> bool {
    return false;
}

fn king_movement(board: &Board, m: &Move, p: PlayerPiece) -> bool {
    return false;
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

