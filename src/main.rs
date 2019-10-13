extern crate rayon;

use rayon::prelude::*;

mod extras;
mod game;
mod point;
mod set;

use extras::{step_1, DirectionIter};
use game::Game;
use point::Point;
use set::{Direction, Set, SetIter};

const DESIRED_SCORE: usize = 13;

fn main() {
    let game = Game::new();
    let mut possible_bests = game
        .points
        .iter().take(1)
        .map(|&point| {
            let point_offset_direction_iter = DirectionIter::new();
            let mut set_direction_iter = DirectionIter::new();
            let mut best = 0;
            let mut game = game.clone();
            'pod: for point_offset_direction in point_offset_direction_iter {
                let start = step_1(point, point_offset_direction);
                if game.points.contains(&start) {
                    continue;
                }
                for set_direction in set_direction_iter {
                    if game.valid_add_set(Set::new(start, set_direction)) {
                        game.add_set(Set::new(start, set_direction));
                        best = best.max(game.score());
                        if best >= DESIRED_SCORE {
                            break 'pod;
                        }
                        if branch(&mut game, &mut best) {
                            break 'pod;
                        } else {
                            let Set { start_x, start_y, direction: _ } = game.sets.pop().unwrap();
                            game.reset();
                            game.apply_current_sets();
                        }
                    }
                }
                set_direction_iter = DirectionIter::new();
            }
            game
        })
        .collect::<Vec<Game>>();
    let mut best = possible_bests.pop().unwrap();
    while let Some(game) = possible_bests.pop() {
        if game.score() > best.score() {
            best = game;
        }
    }
    println!("Got: {}", best.score());
    println!("{:?}", best.sets);
    let s1 = Set::new(Point::new(0, 0), Direction::NE);
    let s2 = Set::new(Point::new(8, 8), Direction::SW);
    println!("{}", s1.acceptable_overlap(s2));
}

// Return value:
// true => hit desired max - return with no further action
// false => pop sets, remove set start
fn branch(game: &mut Game, best: &mut usize) -> bool {
    println!("Start branch ({})", game.score());
    let point_offset_direction_iter = DirectionIter::new();
    let set_direction_iter = DirectionIter::new();
    for point in game.points.clone().into_iter() {
        for point_offset_direction in point_offset_direction_iter {
            let start = step_1(point, point_offset_direction);
            if game.points.contains(&start) {
                continue;
            }
            for set_direction in set_direction_iter {
                if game.valid_add_set(Set::new(start, set_direction)) {
                    game.add_set(Set::new(start, set_direction));
                    *best = (*best).max(game.score());
                    if *best >= DESIRED_SCORE {
                        return true;
                    }
                    if branch(game, best) {
                        return true;
                    } else {
                        let _ = game.sets.pop().unwrap();
                        game.reset();
                        game.apply_current_sets();
                    }
                }
            }
        }
    }
    println!("    Dropping back");
    false
}