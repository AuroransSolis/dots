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

pub fn step_1(point: Point, direction: Direction) -> Point {
    let (diff_x, diff_y) = match direction {
        Direction::N  => ( 0,  1),
        Direction::NE => ( 1,  1),
        Direction::E  => ( 1,  0),
        Direction::SE => ( 1, -1),
        Direction::S  => ( 0, -1),
        Direction::SW => (-1, -1),
        Direction::W  => (-1,  0),
        Direction::NW => (-1,  1)
    };
    Point::new(point.x + diff_x, point.y + diff_y)
}
