extern crate rayon;

use rayon::prelude::*;

use std::collections::HashSet;

mod extras;
mod game;
mod point;
mod set;

use extras::DirectionIter;
use game::Game;
use set::Set;
use crate::point::Point;

const DESIRED_SCORE: usize = 63;

fn main() {
    let game = Game::new();
    let try_best = game
        .points
        .par_iter()
        .map(|&point| base_highest_set(&game, point))
        .find_first(|game| game.score() >= DESIRED_SCORE);
    if let Some(best) = try_best {
        println!("Got: {}", best.score());
        println!("{:?}", best.sets);
    } else {
        println!(":c");
    }
}

fn base_highest_set(game: &Game, point: Point) -> Game {
    // println!("Base point: {}", point);
    let mut game = game.clone();
    let mut possible_moves = HashSet::new();
    for direction in DirectionIter::new() {
        // println!("  direction: {:?}", direction);
        for offset in 0..5 {
            // println!("    offset: {}", offset);
            let set = Set::new(point, direction, offset);
            // println!("  set: {:?}", set);
            if let Some(point) = game.valid_add_set(Set::new(point, direction, offset)) {
                game.add_set(set, point);
                let num_possible_moves = game.possible_moves();
                // println!("    valid set: {}", num_possible_moves);
                possible_moves.insert((num_possible_moves, set, point));
                game.sets.pop();
                game.points.remove(&point);
            }
        }
    }
    // println!("  Found {} moves.", possible_moves.len());
    let mut sorted_possible_moves = Vec::with_capacity(possible_moves.len());
    for m in possible_moves.into_iter() {
        sorted_possible_moves.push(m);
        sorted_possible_moves.sort_unstable_by(|(m1, _, _), (m2, _, _)| m1.cmp(m2));
    }
    // println!("  Sorted possibilities: {:?}", sorted_possible_moves);
    while let Some((_, set, point)) = sorted_possible_moves.pop() {
        game.add_set(set, point);
        if branch_highest_set(&mut game) {
            break;
        } else {
            game.sets.pop();
            game.points.remove(&point);
        }
    }
    game
}

// Return value:
// true => hit desired max - return with no further action
// false => pop sets, remove set start
fn branch_highest_set(game: &mut Game) -> bool {
    // println!("  Begin branch");
    let mut possible_moves = HashSet::new();
    for point in game.points.clone().into_iter() {
        // println!("    Base point: {}", point);
        for direction in DirectionIter::new() {
            // println!("        direction: {:?}", direction);
            for offset in 0..5 {
                // println!("          offset: {}", offset);
                let set = Set::new(point, direction, offset);
                // println!("      set: {:?}", set);
                if let Some(point) = game.valid_add_set(Set::new(point, direction, offset)) {
                    game.add_set(set, point);
                    let num_possible_moves = game.possible_moves();
                    // println!("        valid set: {}", num_possible_moves);
                    possible_moves.insert((num_possible_moves, set, point));
                    game.sets.pop();
                    game.points.remove(&point);
                }
            }
        }
    }
    let mut sorted_possible_moves = Vec::with_capacity(possible_moves.len());
    for m in possible_moves.into_iter() {
        sorted_possible_moves.push(m);
        sorted_possible_moves.sort_unstable_by(|(m1, _, _), (m2, _, _)| m1.cmp(m2));
    }
    // println!("        Sorted possibilities: {:?}", sorted_possible_moves);
    while let Some((_, set, point)) = sorted_possible_moves.pop() {
        game.add_set(set, point);
        if game.score() >= DESIRED_SCORE {
            return true;
        }
        if branch_highest_set(game) {
            return true;
        } else {
            game.sets.pop();
            game.points.remove(&point);
        }
    }
    false
}

// Return value:
// true => hit desired max - return with no further action
// false => pop sets, remove set start
/*fn branch_highest_single(game: &mut Game) -> bool {
    let mut possible_moves = HashSet::new();
    for point in game.points.clone().into_iter() {
        for direction in DirectionIter::new() {
            for offset in 0..5 {
                let set = Set::new(point, direction, offset);
                if let Some(point) = game.valid_add_set(Set::new(point, direction, offset)) {
                    game.add_set(set, point);
                    let num_possible_moves = game.possible_moves();
                    possible_moves.insert((num_possible_moves, set, point));
                    game.sets.pop();
                    game.points.remove(&point);
                }
            }
        }
    }
    let mut sorted_possible_moves = Vec::with_capacity(possible_moves.len());
    for m in possible_moves.into_iter() {
        sorted_possible_moves.push(m);
        sorted_possible_moves.sort_unstable_by(|(m1, _, _), (m2, _, _)| m1.cmp(m2));
    }
    while let Some((_, set, point)) = sorted_possible_moves.pop() {
        game.add_set(set, point);
        if game.score() >= DESIRED_SCORE {
            return true;
        }
        if branch_highest_set(game) {
            return true;
        } else {
            game.sets.pop();
            game.points.remove(&point);
        }
    }
    false
}*/