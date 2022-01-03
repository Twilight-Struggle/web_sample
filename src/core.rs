use serde::{Serialize, Deserialize};
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    cells: Vec<bool>
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: vec![true, false, false]
        }
    }
    pub fn reset(&self) -> Board {
        Board {
            cells: vec![true, false, false]
        }
    }
    pub fn idou(&self, from: usize, to: usize) -> Option<Board> {
        if !self.cells[from] {
            None
        }
        else {
            if to - from == 1 && 2 >= from && 2 >= to {
                let mut cells = vec![false, false, false];
                cells[to] = true;
                Some(Board {cells: cells})
            }
            else {
                None
            }
        }
    }
}