use crate::point::Point;
use std::iter::Iterator;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
// Directions:
//  V |  / SP
//    .  _ H
//
//      \ SN
pub enum Direction {
    H,
    V,
    SP,
    SN
}

impl Direction {
    #[inline]
    pub fn single_step(&self) -> (i16, i16) {
        match self {
            Direction::H => (1, 0),
            Direction::V => (0, 1),
            Direction::SP => (1, 1),
            Direction::SN => (1, -1)
        }
    }

    #[inline]
    pub fn full_step(&self) -> (i16, i16) {
        match self {
            Direction::H => (4, 0),
            Direction::V => (0, 4),
            Direction::SP => (4, 4),
            Direction::SN => (4, -4)
        }
    }

    #[inline]
    pub fn opposite_single_step(&self) -> (i16, i16) {
        match self {
            Direction::H => (-1, 0),
            Direction::V => (0, -1),
            Direction::SP => (-1, -1),
            Direction::SN => (-1, 1)
        }
    }

    #[inline]
    pub fn opposite_full_step(&self) -> (i16, i16) {
        match self {
            Direction::H => (-4, 0),
            Direction::V => (0, -4),
            Direction::SP => (-4, -4),
            Direction::SN => (-4, 4)
        }
    }

    #[inline]
    pub fn set_in_t_mask(self) -> u8 {
        match self {
            Direction::V => 0b00000001,
            Direction::SP => 0b00000010,
            Direction::H => 0b00000100,
            Direction::SN => 0b00001000
        }
    }

    #[inline]
    pub fn set_in_f_mask(self) -> u8 {
        match self {
            Direction::V => 0b11111110,
            Direction::SP => 0b11111101,
            Direction::H => 0b11111011,
            Direction::SN => 0b11110111
        }
    }

    #[inline]
    pub fn set_out_t_mask(self) -> u8 {
        match self {
            Direction::V => 0b00010000,
            Direction::SP => 0b00100000,
            Direction::H => 0b01000000,
            Direction::SN => 0b10000000
        }
    }

    #[inline]
    pub fn set_out_f_mask(self) -> u8 {
        match self {
            Direction::V => 0b11101111,
            Direction::SP => 0b11011111,
            Direction::H => 0b10111111,
            Direction::SN => 0b01111111
        }
    }

    #[inline]
    pub fn set_inout_t_mask(self) -> u8 {
        match self {
            Direction::V => 0b00010001,
            Direction::SP => 0b00100010,
            Direction::H => 0b01000100,
            Direction::SN => 0b10001000
        }
    }

    #[inline]
    pub fn set_inout_f_mask(self) -> u8 {
        match self {
            Direction::V => 0b11101110,
            Direction::SP => 0b11011101,
            Direction::H => 0b10111011,
            Direction::SN => 0b01110111
        }
    }

    #[inline]
    pub fn get_in_mask(self) -> u8 {
        match self {
            Direction::V => 0b00000001,
            Direction::SP => 0b00000010,
            Direction::H => 0b00000100,
            Direction::SN => 0b00001000
        }
    }

    #[inline]
    pub fn get_out_mask(self) -> u8 {
        match self {
            Direction::V => 0b00010000,
            Direction::SP => 0b00100000,
            Direction::H => 0b01000000,
            Direction::SN => 0b10000000
        }
    }

    #[inline]
    pub fn get_inout_mask(self) -> u8 {
        match self {
            Direction::V => 0b00010001,
            Direction::SP => 0b00100010,
            Direction::H => 0b01000100,
            Direction::SN => 0b10001000
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct Set {
    pub(crate) start_x: i16,
    pub(crate) start_y: i16,
    pub(crate) direction: Direction
}

impl Set {
    pub fn new(start: Point, direction: Direction, offset: i16) -> Self {
        let Point {
            x: mut start_x,
            y: mut start_y
        } = start;
        let (step_x, step_y) = direction.opposite_single_step();
        start_x += step_x * offset;
        start_y += step_y * offset;
        Set {
            start_x,
            start_y,
            direction
        }
    }

    pub fn start_point(&self) -> Point {
        Point::new(self.start_x, self.start_y)
    }
}

#[derive(Copy, Clone)]
pub struct SetIter {
    x: i16,
    y: i16,
    dx: i16,
    dy: i16,
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
