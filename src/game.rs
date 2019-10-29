use crate::extras::DirectionIter;
use crate::point::Point;
use crate::set::Set;
use crate::DESIRED_SCORE;
use ahash::{AHashMap, AHashSet};

pub const STARTING_POINTS: [Point; 36] = [
    Point { x: 0, y: 4 },
    Point { x: -1, y: 4 },
    Point { x: -1, y: 3 },
    Point { x: -1, y: 2 },
    Point { x: -1, y: 1 },
    Point { x: -2, y: 1 },
    Point { x: -3, y: 1 },
    Point { x: -4, y: 1 },
    Point { x: -4, y: 0 },
    Point { x: -4, y: -1 },
    Point { x: -4, y: -2 },
    Point { x: -3, y: -2 },
    Point { x: -2, y: -2 },
    Point { x: -1, y: -2 },
    Point { x: -1, y: -3 },
    Point { x: -1, y: -4 },
    Point { x: -1, y: -5 },
    Point { x: 0, y: -5 },
    Point { x: 1, y: -5 },
    Point { x: 2, y: -5 },
    Point { x: 2, y: -4 },
    Point { x: 2, y: -3 },
    Point { x: 2, y: -2 },
    Point { x: 3, y: -2 },
    Point { x: 4, y: -2 },
    Point { x: 5, y: -2 },
    Point { x: 5, y: -1 },
    Point { x: 5, y: 0 },
    Point { x: 5, y: 1 },
    Point { x: 4, y: 1 },
    Point { x: 3, y: 1 },
    Point { x: 2, y: 1 },
    Point { x: 2, y: 2 },
    Point { x: 2, y: 3 },
    Point { x: 2, y: 4 },
    Point { x: 1, y: 4 },
];

#[derive(Clone, Debug)]
pub struct Game {
    pub(crate) points: AHashMap<Point, u8>,
    pub(crate) sets: Vec<Set>,
}

impl Game {
    pub fn new() -> Self {
        let mut points = AHashMap::with_capacity(STARTING_POINTS.len() + DESIRED_SCORE);
        for &point in STARTING_POINTS.iter() {
            points.insert(point, 0);
        }
        Game {
            points,
            sets: Vec::with_capacity(DESIRED_SCORE),
        }
    }

    pub fn add_set(&mut self, set: Set, point: Point) {
        self.sets.push(set);
        self.points.insert(point, 0);
        let masks = [
            set.direction.set_out_t_mask(),
            set.direction.set_inout_t_mask(),
            set.direction.set_inout_t_mask(),
            set.direction.set_inout_t_mask(),
            set.direction.set_in_t_mask(),
        ];
        let mut set_point = set.start_point();
        let step = set.direction.single_step();
        for i in 0..5 {
            let flags = self.points.get_mut(&set_point).unwrap();
            *flags = *flags | masks[i];
            set_point.step(step);
        }
    }

    pub fn add_set_nomod_flags(&mut self, set: Set) {
        self.sets.push(set);
    }

    pub fn remove_set(&mut self, set: Set, point: Point) {
        self.sets.pop();
        self.points.remove(&point);
        let masks = [
            set.direction.set_out_f_mask(),
            set.direction.set_inout_f_mask(),
            set.direction.set_inout_f_mask(),
            set.direction.set_inout_f_mask(),
            set.direction.set_in_f_mask(),
        ];
        let mut set_point = set.start_point();
        let step = set.direction.single_step();
        for i in 0..5 {
            if point != set_point {
                let flags = self.points.get_mut(&set_point).unwrap();
                *flags = *flags & masks[i];
            }
            set_point.step(step);
        }
    }

    pub fn remove_set_nomod_flags(&mut self) {
        self.sets.pop();
    }

    pub fn valid_add_set(&self, test: Set) -> Option<Point> {
        let mut new = None;
        let mut point = test.start_point();
        let step = test.direction.single_step();
        let masks = [
            test.direction.get_out_mask(),
            test.direction.get_inout_mask(),
            test.direction.get_inout_mask(),
            test.direction.get_inout_mask(),
            test.direction.get_in_mask(),
        ];
        for &mask in masks.iter() {
            if let Some(&flags) = self.points.get(&point) {
                if flags & mask > 0 {
                    return None;
                }
            } else if new.is_none() {
                new = Some(point);
            } else {
                return None;
            }
            point.step(step);
        }
        new
    }

    pub fn possible_moves(&self) -> usize {
        let mut moves: AHashSet<Set> = AHashSet::with_capacity(DESIRED_SCORE);
        for (&point, &flags) in self.points.iter() {
            // Point has a set in all directions
            if flags == 255 {
                continue;
            }
            for direction in DirectionIter::new() {
                let (offset_lb, offset_ub) =
                    if (flags & direction.get_inout_mask()).count_ones() == 2 {
                        continue;
                    } else if flags & direction.get_in_mask() > 0 {
                        (0, 1)
                    } else if flags & direction.get_out_mask() > 0 {
                        (4, 5)
                    } else {
                        (0, 5)
                    };
                for offset in offset_lb..offset_ub {
                    let set = Set::new(point, direction, offset);
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
