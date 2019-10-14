use crate::point::Point;
use std::iter::Iterator;
use std::ops::Range;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW
}

impl Direction {
    pub fn parallel_to(&self, other: Direction) -> bool {
        match (self, other) {
            (Direction::N,  Direction::N)  | (Direction::N,  Direction::S)  => true,
            (Direction::NE, Direction::NE) | (Direction::NE, Direction::SW) => true,
            (Direction::E,  Direction::E)  | (Direction::E,  Direction::W)  => true,
            (Direction::SE, Direction::SE) | (Direction::SE, Direction::NW) => true,
            (Direction::S,  Direction::S)  | (Direction::S,  Direction::N)  => true,
            (Direction::SW, Direction::SW) | (Direction::SW, Direction::NE) => true,
            (Direction::W,  Direction::W)  | (Direction::W,  Direction::E)  => true,
            (Direction::NW, Direction::NW) | (Direction::NW, Direction::SE) => true,
            _ => false
        }
    }

    pub fn rot_90_acw(&self) -> Self {
        match self {
            Direction::N  => Direction::W,
            Direction::NE => Direction::NW,
            Direction::E  => Direction::N,
            Direction::SE => Direction::NE,
            Direction::S  => Direction::E,
            Direction::SW => Direction::SE,
            Direction::W  => Direction::S,
            Direction::NW => Direction::SW
        }
    }

    pub fn rot_90_cw(&self) -> Self {
        match self {
            Direction::N  => Direction::E,
            Direction::NE => Direction::SE,
            Direction::E  => Direction::S,
            Direction::SE => Direction::SW,
            Direction::S  => Direction::W,
            Direction::SW => Direction::NW,
            Direction::W  => Direction::N,
            Direction::NW => Direction::NE
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Direction::N  => Direction::S,
            Direction::NE => Direction::SW,
            Direction::E  => Direction::W,
            Direction::SE => Direction::NW,
            Direction::S  => Direction::N,
            Direction::SW => Direction::NE,
            Direction::W  => Direction::E,
            Direction::NW => Direction::SE
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
// Specifies:
// x range: [start_x, end_x)
// y range: [start_y, end_y)
pub struct Set {
    pub(crate) start_x: i32,
    pub(crate) start_y: i32,
    pub(crate) direction: Direction
}

impl Set {
    pub fn new(start: Point, direction: Direction) -> Self {
        let Point {
            x: start_x,
            y: start_y
        } = start;
        Set {
            start_x,
            start_y,
            direction
        }
    }

    pub fn start_point(&self) -> Point {
        Point::new(self.start_x, self.start_y)
    }

    pub fn ranges(&self) -> [Range<i32>; 2] {
        match self.direction {
            Direction::N => [self.start_x..self.start_x, self.start_y..self.start_y + 5],
            Direction::NE => [
                self.start_x..self.start_x + 5,
                self.start_y..self.start_y + 5,
            ],
            Direction::E => [self.start_x..self.start_x + 5, self.start_y..self.start_y],
            Direction::SE => [
                self.start_x..self.start_x + 5,
                self.start_y..self.start_y - 5,
            ],
            Direction::S => [self.start_x..self.start_x, self.start_y..self.start_y - 5],
            Direction::SW => [
                self.start_x..self.start_x - 5,
                self.start_y..self.start_y - 5,
            ],
            Direction::W => [self.start_x..self.start_x - 5, self.start_y..self.start_y],
            Direction::NW => [
                self.start_x..self.start_x - 5,
                self.start_y..self.start_y + 5,
            ]
        }
    }

    pub fn acceptable_overlap(&self, other: Set) -> bool {
        if self.direction.parallel_to(other.direction) {
            // self points iter
            let mut spi = self.iter();
            let self_points = [
                spi.next().unwrap(),
                spi.next().unwrap(),
                spi.next().unwrap(),
                spi.next().unwrap(),
                spi.next().unwrap()
            ];
            // other points iter
            let mut opi = other.iter();
            let mut has_common = false;
            for _ in 0..5 {
                if self_points.contains(&opi.next().unwrap()) {
                    if has_common {
                        return false;
                    } else {
                        has_common = true;
                    }
                }
            }
            true
        } else {
            true
        }
    }

    pub fn iter(&self) -> SetIter {
        let (dx, dy) = match self.direction {
            Direction::N  => ( 0,  1),
            Direction::NE => ( 1,  1),
            Direction::E  => ( 1,  0),
            Direction::SE => ( 1, -1),
            Direction::S  => ( 0, -1),
            Direction::SW => (-1, -1),
            Direction::W  => (-1,  0),
            Direction::NW => (-1,  1)
        };
        SetIter {
            x: self.start_x,
            y: self.start_y,
            dx,
            dy,
            step: 0
        }
    }
}

#[derive(Copy, Clone)]
pub struct SetIter {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    step: u8
}

impl Iterator for SetIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step < 5 {
            let out = Point::new(self.x, self.y);
            self.x += self.dx;
            self.y += self.dy;
            self.step += 1;
            Some(out)
        } else {
            None
        }
    }
}