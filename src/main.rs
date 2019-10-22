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

const DESIRED_SCORE: usize = 58;

fn main() {
    let game = Game::new();
    let try_best = game
        .points
        .par_iter()
        // .iter()
        .map(|(&point, _)| base_highest_set(&game, point))
        .find_first(|game| game.score() >= DESIRED_SCORE);
        // .collect::<Vec<Game>>();
    // try_best.sort_unstable_by(|g1, g2| g1.score().cmp(&g2.score()));
    // let try_best = try_best.pop();
    if let Some(best) = try_best {
        println!("Got: {}", best.score());
        println!("{:?}", best.sets);
    } else {
        println!(":c");
    }
}

fn base_highest_set(game: &Game, point: Point) -> Game {
    let mut game = game.clone();
    // Collect unique possible moves in this hashset
    let mut possible_moves = HashSet::new();
    // println!("Base point: {}", point);
    for direction in DirectionIter::new() {
        for offset in 0..5 {
            let set = Set::new(point, direction, offset);
            // println!("  Set: {:?}", set);
            // print!("a");
            if let Some(point) = game.valid_add_set(Set::new(point, direction, offset)) {
                game.add_set(set, point);
                let num_possible_moves = game.possible_moves();
                // println!("    Valid: {}", num_possible_moves);
                possible_moves.insert((num_possible_moves, set, point));
                game.remove_set(set, point);
            }
        }
    }
    // Collect into a vec to sort
    let mut sorted_possible_moves = Vec::with_capacity(possible_moves.len());
    for m in possible_moves.into_iter() {
        sorted_possible_moves.push(m);
    }
    // Sort by possible moves
    sorted_possible_moves.sort_unstable_by(|(m1, _, _), (m2, _, _)| m1.cmp(m2));
    for (_, set, point) in sorted_possible_moves.into_iter().rev() {
        // println!("Branching ({}, {:?})", point, set);
        // Branch off into recursion-land for each possible move, and break out of the loop if one
        // of the branches meets the required number of moves. Otherwise, if the branch returns and
        // hasn't met the requested number of moves, undo the change made here and try again.
        game.add_set(set, point);
        if branch_highest_set(&mut game) {
            break;
        } else {
            game.remove_set(set, point);
        }
    }
    game
}

// Return value:
// true => hit desired max - return with no further action
// false => pop sets, remove set start
// This thing is basically the same as the base function, except it takes a mutable reference to a
// `Game` that it modifies, and returns a `bool` depending on whether it reached the target number
// of moves. Literally identical otherwise.
fn branch_highest_set(game: &mut Game) -> bool {
    let mut possible_moves = HashSet::new();
    for (point, flags) in game.points.clone().into_iter() {
        // println!("  Base point: {} ({:#010b})", point, flags);
        if flags == 255 {
            continue;
        }
        for direction in DirectionIter::new() {
            if (flags & direction.get_inout_mask()).count_ones() == 2 {
                continue;
            } else if flags & direction.get_out_mask() > 0 {
                break;
            } else {
                for offset in 0..5 {
                    let set = Set::new(point, direction, offset);
                    // println!("    Set: {:?}", set);
                    if let Some(point) = game.valid_add_set(Set::new(point, direction, offset)) {
                        game.add_set(set, point);
                        let num_possible_moves = game.possible_moves();
                        // println!("        Valid: {}", num_possible_moves);
                        possible_moves.insert((num_possible_moves, set, point));
                        game.remove_set(set, point);
                    }
                }
            }
        }
    }
    let mut sorted_possible_moves = Vec::with_capacity(possible_moves.len());
    for m in possible_moves.into_iter() {
        sorted_possible_moves.push(m);
    }
    sorted_possible_moves.sort_unstable_by(|(m1, _, _), (m2, _, _)| m1.cmp(m2));
    for (_, set, point) in sorted_possible_moves.into_iter().rev() {
        game.add_set(set, point);
        if game.score() >= DESIRED_SCORE {
            return true;
        }
        if branch_highest_set(game) {
            return true;
        } else {
            game.remove_set(set, point);
        }
    }
    // Ran out of moves to try, and we didn't reach the requisite number, so return `false`.
    false
}