use crate::point::Point;
use std::iter::Iterator;
use std::ops::Range;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum Direction {
    H,
    V,
    SP,
    SN
}

impl Direction {
    pub fn rot_90(&self) -> Self {
        match self {
            Direction::H => Direction::V,
            Direction::V => Direction::H,
            Direction::SP => Direction::SN,
            Direction::SN => Direction::SP
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
            Direction::H  => ( 1,  0),
            Direction::V  => ( 0,  1),
            Direction::SP => ( 1,  1),
            Direction::SN => ( 1, -1)
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