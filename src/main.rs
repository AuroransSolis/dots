extern crate rayon;

use rayon::prelude::*;

use std::collections::HashSet;

mod extras;
mod game;
mod point;
mod set;

use extras::{step_1, DirectionIter};
use game::Game;
use point::Point;
use set::{Direction, Set, SetIter};

const DESIRED_SCORE: usize = 12;

fn main() {
    let game = Game::new();
    let mut possible_bests = game
        .points
        .iter()//.take(1)
        .map(|&point| {
            println!("Base point: {}", point);
            // let point_offset_direction_iter = DirectionIter::new();
            // let mut set_direction_iter = DirectionIter::new();
            // let mut best = 0;
            let mut game = game.clone();
            let mut possible_moves = HashSet::new();
            // println!("    points: {}", game.points.len());
            'd: for direction in DirectionIter::new() {
                for offset in 0..5 {
                    let set = Set::new(point, direction, offset);
                    if game.valid_add_set(set) {
                        game.add_set(set);
                        let num_possible_moves = game.possible_moves();
                        game.sets.pop();
                    }
                }
                let set_start = step_1(point, direction.opposite());
                let set = Set::new(set_start, direction);
                println!("    D: {:?} | ss: {} | s: {:?}", direction,  set_start, set);
                if game.valid_add_set(set) {
                    game.add_set(set);
                    let num_possible_moves = game.possible_moves();
                    println!("        pm: {}", num_possible_moves);
                    game.sets.pop();
                    game.points.remove(&set_start);
                    possible_moves.insert((num_possible_moves, set));
                }
            }
            let mut sorted_possible_moves = Vec::with_capacity(possible_moves.len());
            for m in possible_moves.into_iter() {
                sorted_possible_moves.push(m);
                sorted_possible_moves.sort_unstable_by(|(m1, _), (m2, _)| m1.cmp(m2));
            }
            println!("    Sorted possibilities: {:?}", sorted_possible_moves);
            while let Some((_, set)) = sorted_possible_moves.pop() {

                game.add_set(set);
                if branch_highest(&mut game) {
                    break;
                } else {
                    game.sets.pop();
                    game.points.remove(&set.start_point());
                }
            }
            /*'pod: for point_offset_direction in point_offset_direction_iter {
                let start = step_1(point, point_offset_direction);
                if game.points.contains(&start) {
                    continue;
                }
                // // println!("    Offset to: {}", start);
                for set_direction in set_direction_iter {
                    if game.valid_add_set(Set::new(start, set_direction)) {
                        let added = game.add_set(Set::new(start, set_direction));
                        best = best.max(game.score());
                        if best >= DESIRED_SCORE {
                            break 'pod;
                        }
                        if branch(&mut game, &mut best) {
                            break 'pod;
                        } else {
                            let _ = game.sets.pop().unwrap();
                            let _ = game.points.remove(&start);
                        }
                    }
                }
                set_direction_iter = DirectionIter::new();
            }*/
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
}

// Return value:
// true => hit desired max - return with no further action
// false => pop sets, remove set start
fn branch_highest(game: &mut Game) -> bool {
    println!("    Begin branch");
    // // println!("        points: {}", game.points.len());
    let mut possible_moves = HashSet::new();
    for point in game.points.clone().into_iter() {
        println!("        Base point: {}", point);
        for direction in DirectionIter::new() {
            let set_start = step_1(point, direction.opposite());
            let set = Set::new(set_start, direction);
            println!("            D: {:?} | ss: {} | s: {:?}", direction,  set_start, set);
            if game.valid_add_set(set) {
                game.add_set(set);
                let num_possible_moves = game.possible_moves();
                println!("                pm: {}", num_possible_moves);
                possible_moves.insert((num_possible_moves, set));
                game.sets.pop();
                game.points.remove(&set_start);
            }
        }
    }
    let mut sorted_possible_moves = Vec::with_capacity(possible_moves.len());
    for m in possible_moves.into_iter() {
        sorted_possible_moves.push(m);
        sorted_possible_moves.sort_unstable_by(|(m1, _), (m2, _)| m1.cmp(m2));
    }
    println!("        Sorted possibilities: {:?}", sorted_possible_moves);
    while let Some((_, set)) = sorted_possible_moves.pop() {
        game.add_set(set);
        if game.score() >= DESIRED_SCORE {
            return true;
        }
        if branch_highest(game) {
            return true;
        } else {
            game.sets.pop();
            game.points.remove(&set.start_point());
        }
    }
    false
}

// Return value:
// true => hit desired max - return with no further action
// false => pop sets, remove set start
fn branch(game: &mut Game, best: &mut usize) -> bool {
    // // println!("    Start branch ({})", game.score());
    let point_offset_direction_iter = DirectionIter::new();
    let set_direction_iter = DirectionIter::new();
    let mut points = Vec::with_capacity(game.points.len());
    for &point in game.points.iter() {
        points.push(point);
    }
    let mut i = 0;
    let mut l = points.len();
    while i < l {
        let point = points[i];
        for point_offset_direction in point_offset_direction_iter {
            let start = step_1(point, point_offset_direction);
            if game.points.contains(&start) || !game.has_possible_set(start) {
                continue;
            }
            // // // println!("Starting point: {}", )
            // Implement hashable connections struct to determine whether a connection can be made?
            for set_direction in set_direction_iter {
                if game.valid_add_set(Set::new(start, set_direction)) {
                    let added = game.add_set(Set::new(start, set_direction));
                    points.push(start);
                    l += 1;
                    *best = (*best).max(game.score());
                    if *best >= DESIRED_SCORE {
                        return true;
                    }
                    if branch(game, best) {
                        return true;
                    } else {
                        // // println!("        Failed ({}): {:?}", game.sets.len(), game.sets);
                        let _ = game.sets.pop();
                        let _ = game.points.remove(&start);
                        let _ = points.pop();
                        l -= 1;
                    }
                }
            }
        }
        i += 1;
    }
    // // println!("        Dropping back");
    false
}