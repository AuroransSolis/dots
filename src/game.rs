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
    Point { x:  1, y:  4 },
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

    pub fn add_set(&mut self, set: Set) -> bool {
        // println!("            Adding {:?}", set);
        self.sets.push(set);
        self.points.insert(set.start_point())
    }

    pub fn valid_add_set(&self, test: Set) -> bool {
        let mut points_iter = test.iter();
        if self.points.contains(&points_iter.next().unwrap()) {
            false
        } else {
            for _ in 0..4 {
                if !self.points.contains(&points_iter.next().unwrap()) {
                    return false;
                }
            }
            for &set in &self.sets {
                if !test.acceptable_overlap(set) {
                    // // println!("    {:?} overlaps {:?}", test, set);
                    return false;
                }
            }
            true
        }
    }

    pub fn possible_moves(&self) -> usize {
        let mut counter = 0;
        for &point in self.points.iter() {
            for direction in DirectionIter::new() {
                let set_start = step_1(point, direction.opposite());
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
