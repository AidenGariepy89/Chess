use anyhow::{Result, anyhow};
use super::{utils::{Piece, Player, Move}, board::{BOARD_LEN, Board}};

pub struct Snapshot {
    spaces: [Piece; BOARD_LEN],
}

impl Snapshot {
    pub fn snap(board: &Board, m: &Move) -> Self {
        let mut spaces = board.get_spaces();
        spaces[m.to] = spaces[m.from];
        spaces[m.from] = Piece::None;

        return Snapshot { spaces };
    }
}

pub fn is_in_check(snap: &Snapshot, turn: Player) -> Result<()>  {

    return Err(anyhow!("Not implemented yet!"));
}

