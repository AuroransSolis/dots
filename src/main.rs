extern crate ahash;
extern crate num_cpus;
extern crate rayon;

mod extras;
mod game;
mod point;
mod set;

use ahash::{AHashSet, AHashMap};
use extras::DirectionIter;
use game::{Game, STARTING_POINTS};
use point::Point;
use rayon::prelude::*;
use set::Set;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, TryRecvError};
use std::thread::spawn;

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
    // let gamestates = vec![Vec::new(); DESIRED_SCORE];
    /*let try_best = STARTING_POINTS
        .par_iter()
        .map(|&point| base_highest_set(gamestates.clone(), Game::new(), point))
        .find_first(|game| game.score() >= DESIRED_SCORE);*/
    let (send, recv) = channel();
    let starting_points = Arc::new(Mutex::new(STARTING_POINTS.iter().cloned()));
    for i in 0..num_cpus::get() {
        let mut t_gamestates = vec![Vec::new(); DESIRED_SCORE];;
        let t_send = send.clone();
        let t_starting_points = starting_points.clone();
        spawn(move || {
            loop {
                let point = {
                    let mut lock = t_starting_points
                        .lock()
                        .expect("Failed to get lock on starting points iterator.");
                    lock.next()
                };
                if let Some(point) = point {
                    t_send.send((
                        base_highest_set(&mut t_gamestates,Game::new(), point),
                        t_gamestates.clone()
                    )).unwrap();
                } else {
                    break;
                }
            }
        });
        println!("Starting {}", i);
    }
    drop(send);
    let mut try_best = None;
    loop {
        match recv.try_recv() {
            Ok((game, gamestates)) => if game.score() >= DESIRED_SCORE {
                try_best = Some(game);
                let mut num_gamestates = 0;
                for (i, gamestates_of_size_n) in gamestates.iter().enumerate() {
                    println!("{}: {}", i + 1, gamestates_of_size_n.len());
                    num_gamestates += gamestates_of_size_n.len();
                }
                println!("Total: {}", num_gamestates);
                break;
            } else {
                continue;
            },
            Err(TryRecvError::Disconnected) => break,
            _ => continue
        }
    }
    if let Some(best) = try_best {
        println!("Got: {}", best.score());
    } else {
        println!(":c");
    }
}

fn base_highest_set(
    gamestates: &mut Vec<Vec<AHashSet<Set>>>,
    mut game: Game,
    point: Point
) -> Game {
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
                if !gamestates[game.score() - 1]
                    .iter()
                    .any(|gs| v_hs_eq(&game.sets, gs))
                {
                    let mut new_gamestate = AHashSet::with_capacity(game.score());
                    for &set in &game.sets {
                        new_gamestate.insert(set);
                    }
                    gamestates[game.score() - 1].push(new_gamestate);
                    let num_possible_moves = game.possible_moves();
                    possible_moves.insert((num_possible_moves, set, point));
                }
                game.remove_set(set, point);
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
        if game.score() >= DESIRED_SCORE {
            break;
        }
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
fn branch_highest_set(gamestates: &mut Vec<Vec<AHashSet<Set>>>, game: &mut Game) -> bool {
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
                    if !gamestates[game.score() - 1]
                        .iter()
                        .any(|gs| v_hs_eq(&game.sets, gs))
                    {
                        let mut new_gamestate = AHashSet::with_capacity(game.score());
                        for &set in &game.sets {
                            new_gamestate.insert(set);
                        }
                        gamestates[game.score() - 1].push(new_gamestate);
                        let num_possible_moves = game.possible_moves();
                        possible_moves.insert((num_possible_moves, set, point));
                    }
                    game.remove_set(set, point);
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
        if branch_highest_set(gamestates, game) {
            return true;
        } else {
            game.remove_set(set, point);
        }
    }
    // Ran out of moves to try, and we didn't reach the requisite number, so return `false`.
    false
}

fn v_hs_eq(v: &Vec<Set>, hs: &AHashSet<Set>) -> bool {
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
