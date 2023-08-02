use crate::grid_cell::Cell;

use std::cmp::Ordering;

#[derive(Clone, Copy)]
pub struct Node {
    pub cell: Cell,
    pub g: f64,
    pub f: f64,
}

impl Node {
    pub fn new(cell: Cell, g: f64, h: f64) -> Node {
        Node { cell, g, f: g + h }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.f.partial_cmp(&self.f)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.partial_cmp(&self.f).unwrap()
    }
}
