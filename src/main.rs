extern crate ahash;
extern crate rayon;

use rayon::prelude::*;

use std::collections::HashSet;

mod extras;
mod game;
mod point;
mod set;

use ahash::AHashSet;
use extras::DirectionIter;
use game::{Game, STARTING_POINTS};
use point::Point;
use set::Set;
use std::sync::{Arc, Mutex};

const DESIRED_SCORE: usize = 85;

fn main() {
    // Aight, so an explanation. I'm going to ignore the `Arc<Mutex<>>` part since that's just
    // making multithreaded mutability safe. Onto the `Vec<Vec<Hashmap<Set>>>`, then. So what I'm
    // doing with this is storing each unique gamestate I encounter, and then "sorting" those by
    // length. So gamestates[0] contains all the gamestates with 1 move I've encountered,
    // gamestates[1] contains all the gamestates with 2 moves I've encountered, and so on. This is
    // so that I can avoid going down a branch of recursion if I've already found all of its ends.
    // And from how my recursive algorithm works, if I encounter a complete set of moves again then
    // I know that I've already tried all of the remaining possibilities for that gamestate.
    let mut gamestates = Arc::new(Mutex::new(vec![Vec::new(); DESIRED_SCORE]));
    let try_best = STARTING_POINTS
        // .iter()
        .par_iter()
        .map(|&point| base_highest_set(&gamestates, Game::new(), point))
        .find_first(|game| game.score() >= DESIRED_SCORE);
    // .take_while(|game| game.score() <= DESIRED_SCORE)
    // .next();
    if let Some(best) = try_best {
        println!("Got: {}", best.score());
        println!("{:?}", best.sets);
    } else {
        println!(":c");
    }
    let mut num_gamestates = 0;
    for (i, gamestates_of_size_n) in gamestates.iter().enumerate() {
        println!("{}: {}", i + 1, gamestates_of_size_n.len());
        num_gamestates += gamestates_of_size_n.len();
    }
    println!("Total: {}", num_gamestates);
}

fn base_highest_set(
    gamestates: &Arc<Mutex<Vec<Vec<HashSet<Set>>>>>,
    mut game: Game,
    point: Point,
) -> Game {
    let gamestates = gamestates.clone();
    // Collect unique possible moves in this hashset
    let mut possible_moves: AHashSet<(usize, Set, Point)> = AHashSet::with_capacity(DESIRED_SCORE);
    let flags = *game.points.get(&point).expect("foo");
    for direction in DirectionIter::new() {
        let (offset_lb, offset_ub) = if (flags & direction.get_inout_mask()).count_ones() == 2 {
            // If this point is being used in and out in a given direction, it can no longer be the
            // start or end point for a set. Move on to the next direction.
            continue;
        } else if flags & direction.get_in_mask() > 0 {
            // If this point is being used coming into the point from a given direction, this point
            // and the next four in the same direction can be part of a set.
            (0, 1)
        } else if flags & direction.get_out_mask() > 0 {
            // If this point is being used going out of the point from a given direction, this point
            // and the next four in the same direction (but stepping backwards) can be part of a
            // set.
            (4, 5)
        } else {
            // This point can be the start, end, or in the middle of a set.
            (0, 5)
        };
        for offset in offset_lb..offset_ub {
            let set = Set::new(point, direction, offset);
            if let Some(point) = game.valid_add_set(Set::new(point, direction, offset)) {
                game.add_set(set, point);
                if !gamestates
                    .lock()
                    .expect("Failed to get lock on gamestates.")[game.score() - 1]
                    .iter()
                    .any(|gs| v_hs_eq(&game.sets, gs))
                {
                    let num_possible_moves = game.possible_moves();
                    possible_moves.insert((num_possible_moves, set, point));
                    game.remove_set(set, point);
                }
            }
        }
    }
    // Collect into a vec to sort
    let mut sorted_possible_moves = Vec::with_capacity(DESIRED_SCORE);
    for m in possible_moves.into_iter() {
        sorted_possible_moves.push(m);
    }
    // Sort by possible moves
    sorted_possible_moves.sort_unstable_by(|(m1, _, _), (m2, _, _)| m1.cmp(m2));
    for (_, set, point) in sorted_possible_moves.into_iter().rev() {
        // Branch off into recursion-land for each possible move, and break out of the loop if one
        // of the branches meets the required number of moves. Otherwise, if the branch returns and
        // hasn't met the requested number of moves, undo the change made here and try again. We
        // also don't have to worry about filtering the moves we've collected here since that's
        // already been done in the possible move collection loop.
        game.add_set(set, point);
        if branch_highest_set(gamestates, &mut game) {
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
fn branch_highest_set(gamestates: Arc<Mutex<Vec<Vec<HashSet<Set>>>>>, game: &mut Game) -> bool {
    let mut possible_moves: AHashSet<(usize, Set, Point)> = AHashSet::with_capacity(DESIRED_SCORE);
    for (point, flags) in game.points.clone().into_iter() {
        if flags == 255 {
            continue;
        }
        for direction in DirectionIter::new() {
            let (offset_lb, offset_ub) = if (flags & direction.get_inout_mask()).count_ones() == 2 {
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
                if let Some(point) = game.valid_add_set(Set::new(point, direction, offset)) {
                    game.add_set(set, point);
                    if !gamestates
                        .lock()
                        .expect("Failed to get lock on gamestates.")[game.score() - 1]
                        .iter()
                        .any(|gs| v_hs_eq(&game.sets, gs))
                    {
                        let num_possible_moves = game.possible_moves();
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
        {
            let lock = gamestates.lock().expect("Failed to get lock on gamestates.");
            if !lock[game.score() - 1]
                .iter()
                .any(|gs| v_hs_eq(&game.sets, gs))
            {
                let mut gamestate = HashSet::with_capacity(game.score());
                for &set in game.sets.iter() {
                    gamestate.insert(set);
                }
                lock[game.score() - 1].push(gamestate);
            }
        }
        if game.score() >= DESIRED_SCORE {
            return true;
        }
        if branch_highest_set(gamestates.clone(), game) {
            return true;
        } else {
            game.remove_set(set, point);
        }
    }
    // Ran out of moves to try, and we didn't reach the requisite number, so return `false`.
    false
}

fn v_hs_eq(v: &Vec<Set>, hs: &HashSet<Set>) -> bool {
    if v.len() == hs.len() {
        for item in v.iter() {
            if !hs.contains(item) {
                return false;
            }
        }
        true
    } else {
        false
    }
}
