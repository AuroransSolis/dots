use crate::extras::DirectionIter;
use crate::point::Point;
use crate::set::Set;
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
        self.sets.push(set);
        self.points.insert(point)
    }

    pub fn valid_add_set(&self, test: Set) -> Option<Point> {
        // println!("vas");
        let mut new = None;
        for point in test.iter() {
            // println!("  point: {}", point);
            if self.points.contains(&point) {
                // println!("    c");
                continue;
            } else if new.is_none() {
                // println!("    first nc");
                new = Some(point);
            } else {
                // println!("    second nc");
                return None;
            }
        }
        if new.is_some() {
            for &set in &self.sets {
                // println!("  {:?} overlaps with {:?}?", test, set);
                if !test.acceptable_overlap(set) {
                    // println!("    yes");
                    return None;
                }
            }
            new
        } else {
            None
        }
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
