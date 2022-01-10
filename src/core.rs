use serde::{Serialize, Deserialize};
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Default)]
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
        else if to > from && to - from == 1 && 2 >= from && 2 >= to {
            let mut cells = vec![false, false, false];
            cells[to] = true;
            Some(Board {cells})
        }
        else {
            None
        }
    }
    pub fn goaled(&self) -> bool {
        self.cells[2]
    }
}