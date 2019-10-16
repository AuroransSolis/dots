use crate::extras::DirectionIter;
use crate::point::Point;
use crate::set::{Direction, Set};
use std::collections::{HashMap, HashSet};

const STARTING_POINTS: [Point; 36] = [
    Point { x:  0, y:  4 },
    Point { x: -1, y:  4 },
    Point { x: -1, y:  3 },
    Point { x: -1, y:  2 },
    Point { x: -1, y:  1 },
    Point { x: -2, y:  1 },
    Point { x: -3, y:  1 },
    Point { x: -4, y:  1 },
    Point { x: -4, y:  0 },
    Point { x: -4, y: -1 },
    Point { x: -4, y: -2 },
    Point { x: -3, y: -2 },
    Point { x: -2, y: -2 },
    Point { x: -1, y: -2 },
    Point { x: -1, y: -3 },
    Point { x: -1, y: -4 },
    Point { x: -1, y: -5 },
    Point { x:  0, y: -5 },
    Point { x:  1, y: -5 },
    Point { x:  2, y: -5 },
    Point { x:  2, y: -4 },
    Point { x:  2, y: -3 },
    Point { x:  2, y: -2 },
    Point { x:  3, y: -2 },
    Point { x:  4, y: -2 },
    Point { x:  5, y: -2 },
    Point { x:  5, y: -1 },
    Point { x:  5, y:  0 },
    Point { x:  5, y:  1 },
    Point { x:  4, y:  1 },
    Point { x:  3, y:  1 },
    Point { x:  2, y:  1 },
    Point { x:  2, y:  2 },
    Point { x:  2, y:  3 },
    Point { x:  2, y:  4 },
    Point { x:  1, y:  4 }
];

#[derive(Clone, Debug)]
pub struct Game {
    pub(crate) points: HashMap<Point, u8>,
    pub(crate) sets: Vec<Set>
}

impl Game {
    pub fn new() -> Self {
        let mut points = HashMap::new();
        for &point in STARTING_POINTS.iter() {
            points.insert(point, 0);
        }
        Game {
            points,
            sets: Vec::new()
        }
    }

    pub fn set_in(&mut self, point: Point, d: Direction, tf: bool) {
        if tf {
            let flags = self.points.get_mut(&point).unwrap();
            *flags = *flags | d.set_in_t_mask();
        } else {
            let flags = self.points.get_mut(&point).unwrap();
            *flags = *flags & d.set_in_f_mask();
        }
    }

    pub fn get_in(&self, point: Point, d: Direction) -> bool {
        let flags = self.points.get(&point).unwrap();
        *flags & d.get_in_mask() > 0
    }

    pub fn set_out(&mut self, point: Point, d: Direction, tf: bool) {
        if tf {
            let flags = self.points.get_mut(&point).unwrap();
            *flags = *flags | d.set_out_t_mask();
        } else {
            let flags = self.points.get_mut(&point).unwrap();
            *flags = *flags & d.set_out_f_mask();
        }
    }

    pub fn get_out(&self, point: Point, d: Direction) -> bool {
        let flags = self.points.get(&point).unwrap();
        *flags & d.get_out_mask() > 0
    }

    pub fn add_set(&mut self, set: Set, point: Point, flags: u8) {
        self.sets.push(set);
        self.points.insert(point, flags);
    }

    pub fn valid_add_set(&self, test: Set) -> Option<(Point, i32)> {
        let mut new = None;
        let mut loc = -1;
        let mut set_iter = test.iter();
        let first = set_iter.next().unwrap();
        if let Some(&flags) = self.points.get(&first) {
            if self.get_out(first, test.direction) {
                return None;
            }
        } else {
            loc = 0;
            new = Some(first);
        }
        for n in 1..4 {
            let next = set_iter.next().unwrap();
            if let Some(&flags) = self.points.get(&next) {
                if self.get_in(next, test.direction) || self.get_out(next, test.direction) {
                    return None;
                }
            } else {
                if new.is_none() {
                    loc = i;
                    new = Some(next);
                } else {
                    return None;
                }
            }
        }
        let last = set_iter.next().unwrap();
        if let Some(&flags) = self.points.get(&last) {
            if self.get_in(last, test.direction) {
                return None;
            }
        } else {
            if new.is_none() {
                loc = i;
                new = Some(last);
            } else {
                return None;
            }
        }
        Some((new.unwrap(), loc))
    }

    pub fn possible_moves(&self) -> usize {
        // println!("Possible moves");
        // println!("  points: {}", self.points.len());
        let mut moves = HashSet::new();
        for &point in self.points.iter() {
            // println!("  Point: {}", point);
            for direction in DirectionIter::new() {
                for offset in 0..5 {
                    let set = Set::new(point, direction, offset);
                    // println!("    Set: {:?}", set);
                    if self.valid_add_set(set).is_some() {
                        moves.insert(set);
                    }
                }
            }
        }
        moves.len()
    }

    pub fn score(&self) -> usize {
        self.sets.len()
    }
}
