use crate::extras::{DirectionIter, step_1};
use crate::point::Point;
use crate::set::{Direction, Set};
use std::collections::HashSet;

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
    pub(crate) points: HashSet<Point>,
    pub(crate) sets: Vec<Set>
}

impl Game {
    pub fn new() -> Self {
        let mut points = HashSet::new();
        for &point in STARTING_POINTS.iter() {
            points.insert(point);
        }
        Game {
            points,
            sets: Vec::new()
        }
    }

    pub fn add_set(&mut self, set: Set, point: Point) -> bool {
        // println!("            Adding {:?}", set);
        self.sets.push(set);
        self.points.insert(point)
    }

    pub fn valid_add_set(&self, test: Set, add_point: Point) -> bool {
        if self.points.contains(&add_point) {
            false
        } else {
            let mut contains = 0;
            for point in test.iter() {
                if point == add_point {
                    continue;
                } else if self.points.contains(&point) {
                    contains += 1;
                }
            }
            if contains == 4 {
                for &set in &self.sets {
                    if !test.acceptable_overlap(set) {
                        return false
                    }
                }
                true
            } else {
                false
            }
        }
    }

    pub fn possible_moves(&self) -> usize {
        let mut moves = HashSet::new();
        for &point in self.points.iter() {
            for direction in DirectionIter::new() {
                for offset in 0..5 {
                    let set = Set::new(point, direction, offset);
                    if self.valid_add_set(set, point) {

                    }
                }
                let set = Set::new(set_start, direction);
                if self.valid_add_set(set) {
                    counter += 1;
                }
            }
        }
        counter
    }

    pub fn has_possible_set(&self, point: Point) -> bool {
        for direction in DirectionIter::new() {
            if self.valid_add_set(Set::new(point, direction)) {
                return true;
            }
        }
        false
    }

    pub fn reset(&mut self) {
        self.points.clear();
        for &point in STARTING_POINTS.iter() {
            self.points.insert(point);
        }
    }

    pub fn apply_current_sets(&mut self) {
        for set in self.sets.iter() {
            self.points.insert(set.start_point());
        }
    }

    pub fn score(&self) -> usize {
        self.sets.len()
    }
}
