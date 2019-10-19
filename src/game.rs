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
        let flags = self.points.get_mut(&point).unwrap();
        if tf {
            *flags = *flags | d.set_in_t_mask();
        } else {
            *flags = *flags & d.set_in_f_mask();
        }
    }

    pub fn set_out(&mut self, point: Point, d: Direction, tf: bool) {
        let flags = self.points.get_mut(&point).unwrap();
        if tf {
            *flags = *flags | d.set_out_t_mask();
        } else {
            *flags = *flags & d.set_out_f_mask();
        }
    }

    pub fn set_inout(&mut self, point: Point, d: Direction, tf: bool) {
        let flags = self.points.get_mut(&point).unwrap();
        if tf {
            *flags = *flags | d.set_inout_t_mask();
        } else {
            *flags = *flags & d.set_inout_f_mask();
        }
    }

    pub fn get_in(&self, point: Point, d: Direction) -> bool {
        let flags = self.points.get(&point).unwrap();
        *flags & d.get_in_mask() > 0
    }

    pub fn get_out(&self, point: Point, d: Direction) -> bool {
        let flags = self.points.get(&point).unwrap();
        *flags & d.get_out_mask() > 0
    }

    pub fn get_inout(&self, point: Point, d: Direction) -> bool {
        let flags = self.points.get(&point).unwrap();
        (*flags & d.get_inout_mask()).count_ones() == 2
    }

    pub fn add_set(&mut self, set: Set, point: Point) {
        self.sets.push(set);
        self.points.insert(point, 0);
        let masks = [
            set.direction.set_out_t_mask(),
            set.direction.set_inout_t_mask(),
            set.direction.set_inout_t_mask(),
            set.direction.set_inout_t_mask(),
            set.direction.set_in_t_mask()
        ];
        let mut point = set.start_point();
        let step = set.direction.single_step();
        for i in 0..5 {
            let flags = self.points.get_mut(&point).unwrap();
            *flags = *flags | masks[i];
            point.step(step);
        }
    }

    pub fn remove_set(&mut self, set: Set, point: Point) {
        self.sets.push(set);
        self.points.remove(&point);
        let masks = [
            set.direction.set_out_f_mask(),
            set.direction.set_inout_f_mask(),
            set.direction.set_inout_f_mask(),
            set.direction.set_inout_f_mask(),
            set.direction.set_in_f_mask()
        ];
        let mut set_point = set.start_point();
        let step = set.direction.single_step();
        for i in 0..5 {
            if point == set_point {
                set_point.step(step);
                continue;
            }
            let flags = self.points.get_mut(&set_point).unwrap();
            *flags = *flags & masks[i];
            set_point.step(step);
        }
    }

    pub fn valid_add_set(&self, test: Set) -> Option<(Point, u8)> {
        let mut new = None;
        let mut which = 6;
        let mut point = test.start_point();
        let masks = [
            test.direction.get_out_mask(),
            test.direction.get_inout_mask(),
            test.direction.get_inout_mask(),
            test.direction.get_inout_mask(),
            test.direction.get_in_mask()
        ];
        for (loc, &mask) in masks.iter().enumerate() {
            if let Some(&flags) = self.points.get(&point) {
                if flags & mask > 0 {
                    return None;
                }
            } else if which == 6 {
                new = Some(point);
                which = loc as u8;
            } else {
                return None;
            }
        }
        Some((new.unwrap(), which))
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