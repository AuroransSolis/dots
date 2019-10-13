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

    pub fn add_set(&mut self, set: Set) {
        println!("        Adding {:?}", set);
        self.points.insert(set.start_point());
        self.sets.push(set);
    }

    pub fn valid_add_set(&self, test: Set) -> bool {
        let mut contained = 0;
        for point in test.iter().skip(1) {
            // println!("    Contains {}? {}", point, self.points.contains(&point));
            if self.points.contains(&point) {
                contained += 1;
            }
        }
        // println!("    Set contains {} points of requisite 4", contained);
        if contained == 4 {
            for &set in &self.sets {
                if !test.acceptable_overlap(set) {
                    // println!("    {:?} overlaps {:?}", test, set);
                    return false;
                }
            }
            println!("    Sets ({}): {:?}", self.sets.len(), self.sets);
            true
        } else {
            false
        }
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
