use crate::set::Direction;
use std::iter::Iterator;

const DIRECTIONS: [Direction; 4] = [Direction::V, Direction::SP, Direction::H, Direction::SN];

#[derive(Copy, Clone)]
pub struct DirectionIter {
    d: u8,
}

impl Iterator for DirectionIter {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.d < 4 {
            let out = DIRECTIONS[self.d as usize];
            self.d += 1;
            Some(out)
        } else {
            None
        }
    }
}

impl DirectionIter {
    #[inline]
    pub fn new() -> Self {
        DirectionIter { d: 0 }
    }
}

/*#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct Connections {
    n: bool,
    ne: bool,
    e: bool,
    se: bool,
    s: bool,
    sw: bool,
    w: bool,
    nw: bool
}

impl Connections {
    pub fn new(d: Direction) -> Self {

    }
}*/
