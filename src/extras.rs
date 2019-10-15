use crate::point::Point;
use crate::set::Direction;
use std::iter::Iterator;

const DIRECTIONS: [Direction; 8] = [
    Direction::N,
    Direction::NE,
    Direction::E,
    Direction::SE,
    Direction::S,
    Direction::SW,
    Direction::W,
    Direction::NW,
];

#[derive(Copy, Clone)]
pub struct DirectionIter {
    d: u8
}

impl Iterator for DirectionIter {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.d < 8 {
            let out = DIRECTIONS[self.d as usize];
            self.d += 1;
            Some(out)
        } else {
            None
        }
    }
}

impl DirectionIter {
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