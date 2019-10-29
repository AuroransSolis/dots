use ahash::AHashSet;
use crate::DESIRED_SCORE;
use crate::extras::DirectionIter;
use crate::game::Game;
use crate::point::Point;
use crate::set::Set;
use std::cmp::Ordering;
use std::hash::Hash;
use std::iter::{Iterator, Map, Rev};
use std::vec::IntoIter;

pub struct Method<MoveSet, MoveInfo, MoveIter>
    where
        MoveInfo: Hash + Eq + PartialEq,
        MoveIter: Iterator<Item = (Set, Point)>
{
    set_comparison: fn(&MoveSet, &Vec<Set>) -> bool,
    store_new_set: fn(&mut Vec<Vec<MoveSet>>, &Vec<Set>),
    store_move_info: fn(&mut AHashSet<MoveInfo>, usize, Set, Point),
    sort_move_info: fn(&MoveInfo, &MoveInfo) -> Ordering,
    move_iter: fn(Vec<MoveInfo>) -> MoveIter
}

impl<MoveSet, MoveInfo: Hash + Eq + PartialEq, MoveIter> Method<MoveSet, MoveInfo, MoveIter>
    where MoveIter: Iterator<Item = (Set, Point)>
{
    pub fn new(
        set_comparison: fn(&MoveSet, &Vec<Set>) -> bool,
        store_new_set: fn(&mut Vec<Vec<MoveSet>>, &Vec<Set>),
        store_move_info: fn(&mut AHashSet<MoveInfo>, usize, Set, Point),
        sort_move_info: fn(&MoveInfo, &MoveInfo) -> Ordering,
        move_iter: fn(Vec<MoveInfo>) -> MoveIter
    ) -> Self {
        Method {
            set_comparison,
            store_new_set,
            store_move_info,
            sort_move_info,
            move_iter
        }
    }
}

impl<MoveSet, MoveInfo, MoveIter> Clone for Method<MoveSet, MoveInfo, MoveIter>
    where
        MoveInfo: Hash + Eq + PartialEq,
        MoveIter: Iterator<Item = (Set, Point)>
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<MoveSet, MoveInfo, MoveIter> Copy for Method<MoveSet, MoveInfo, MoveIter>
    where
        MoveInfo: Hash + Eq + PartialEq,
        MoveIter: Iterator<Item = (Set, Point)>
{}

pub fn base<MoveSet, MoveInfo: Hash + Eq + PartialEq, MoveIter: Iterator<Item = (Set, Point)>>(
    gamestates: &mut Vec<Vec<MoveSet>>,
    mut game: Game,
    point: Point,
    method: Method<MoveSet, MoveInfo, MoveIter>
) -> Game {
    let Method {
        set_comparison,
        store_new_set,
        store_move_info,
        sort_move_info,
        move_iter
    } = method;
    let mut possible_moves: AHashSet<MoveInfo> = AHashSet::with_capacity(DESIRED_SCORE);
    let flags = *game.points.get(&point).unwrap();
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
            if let Some(point) = game.valid_add_set(set) {
                game.add_set(set, point);
                if !gamestates[game.score() - 1]
                    .iter()
                    .any(|gamestate| set_comparison(gamestate, &game.sets))
                {
                    store_new_set(gamestates, &game.sets);
                    game.remove_set_nomod_flags();
                    game.add_set(set, point);
                    let num_possible_moves = game.possible_moves();
                    store_move_info(&mut possible_moves, num_possible_moves, set, point);
                    game.remove_set(set, point);
                } else {
                    game.remove_set_nomod_flags();
                }
            }
        }
    }
    let mut sorted_possible_moves = Vec::with_capacity(possible_moves.len());
    for item in possible_moves.into_iter() {
        sorted_possible_moves.push(item);
    }
    sorted_possible_moves.sort_unstable_by(sort_move_info);
    for (set, point) in move_iter(sorted_possible_moves) {
        game.add_set(set, point);
        if game.score() >= DESIRED_SCORE {
            break;
        }
        if branch::<MoveSet, MoveInfo, MoveIter>(gamestates, &mut game, method) {
            break;
        } else {
            game.remove_set(set, point);
        }
    }
    game
}

fn branch<MoveSet, MoveInfo: Hash + Eq + PartialEq, MoveIter: Iterator<Item = (Set, Point)>>(
    gamestates: &mut Vec<Vec<MoveSet>>,
    game: &mut Game,
    method: Method<MoveSet, MoveInfo, MoveIter>
) -> bool {
    let Method {
        set_comparison,
        store_new_set,
        store_move_info,
        sort_move_info,
        move_iter
    } = method;
    let mut possible_moves: AHashSet<MoveInfo> = AHashSet::with_capacity(DESIRED_SCORE);
    for (point, flags) in game.points.clone().into_iter() {
        if flags == 255 {
            continue;
        }
        for direction in DirectionIter::new() {
            let (offset_lb, offset_ub) = if(flags & direction.get_inout_mask()).count_ones() == 2 {
                continue;
            } else if flags & direction.get_in_mask() > 0 {
                (0, 1)
            } else if flags & direction.get_in_mask() > 0 {
                (4, 5)
            } else {
                (0, 5)
            };
            for offset in offset_lb..offset_ub {
                let set = Set::new(point, direction, offset);
                if let Some(point) = game.valid_add_set(set) {
                    game.add_set_nomod_flags(set);
                    if !gamestates[game.score() - 1]
                        .iter()
                        .any(|gamestate| set_comparison(gamestate, &game.sets))
                    {
                        store_new_set(gamestates, &game.sets);
                        game.remove_set_nomod_flags();
                        game.add_set(set, point);
                        let num_possible_moves = game.possible_moves();
                        store_move_info(&mut possible_moves, num_possible_moves, set, point);
                        game.remove_set(set, point);
                    } else {
                        game.remove_set_nomod_flags();
                    }
                }
            }
        }
    }
    let mut sorted_possible_moves = Vec::with_capacity(possible_moves.len());
    for item in possible_moves.into_iter() {
        sorted_possible_moves.push(item);
    }
    sorted_possible_moves.sort_unstable_by(sort_move_info);
    for (set, point) in move_iter(sorted_possible_moves) {
        game.add_set(set, point);
        if game.score() >= DESIRED_SCORE {
            return true;
        }
        if branch::<MoveSet, MoveInfo, MoveIter>(gamestates, game, method) {
            return true;
        } else {
            game.remove_set(set, point);
        }
    }
    false
}

// h ighest
// p ossible
// m moves
pub fn cmp_hpm(v1: &(usize, Set, Point), v2: &(usize, Set, Point)) -> Ordering {
    v1.0.cmp(&v2.0)
}

// Primary: mnx, secondary: hpm
// m ost
// n egative
// x coordinate
pub fn cmp_mnx_hpm(v1: &(usize, Set, Point), v2: &(usize, Set, Point)) -> Ordering {
    match v1.2.x.cmp(&v2.2.x) {
        Ordering::Equal => v1.0.cmp(&v2.0),
        cmp @ _ => cmp,
    }
}

// Primary: quad, secondary: hpm
// quad: quadrant
pub fn cmp_quad_hpm(v1: &(usize, Set, Point), v2: &(usize, Set, Point)) -> Ordering {
    match v1.1.start_point().quadrant().cmp(&v2.1.start_point().quadrant()) {
        Ordering::Equal => v1.0.cmp(&v2.0),
        cmp @ _ => cmp,
    }
}

pub fn store_hs(all_gamestates: &mut Vec<Vec<AHashSet<Set>>>, new_gamestate: &Vec<Set>) {
    let mut store = AHashSet::with_capacity(new_gamestate.len());
    for &set in new_gamestate.iter() {
        store.insert(set);
    }
    all_gamestates[new_gamestate.len() - 1].push(store);
}

pub fn store_v(all_gamestates: &mut Vec<Vec<Vec<Set>>>, new_gamestate: &Vec<Set>) {
    all_gamestates[new_gamestate.len() - 1].push(new_gamestate.clone());
}

pub fn store_npm_s_p(
    moves_info: &mut AHashSet<(usize, Set, Point)>,
    num_possible_moves: usize,
    set: Set,
    point: Point
) {
    moves_info.insert((num_possible_moves, set, point));
}

pub fn spm_intoiter(sorted_possible_moves: Vec<(usize, Set, Point)>)
    -> impl Iterator<Item = (Set, Point)>
{
    sorted_possible_moves.into_iter().map(|(_, set, point)| (set, point))
}

pub fn spm_intoiter_rev(sorted_possible_moves: Vec<(usize, Set, Point)>)
    -> impl Iterator<Item = (Set, Point)>
{
    sorted_possible_moves.into_iter().rev().map(|(_, set, point)| (set, point))
}

pub fn base_highest_set(
    gamestates: &mut Vec<Vec<AHashSet<Set>>>,
    mut game: Game,
    point: Point
) -> Game {
    // Collect unique possible moves in this hashset
    let mut possible_moves: AHashSet<(usize, Set, Point)> = AHashSet::with_capacity(DESIRED_SCORE);
    // let mut possible_moves: AHashSet<(usize, Set, Point)> = AHashSet::with_capacity(DESIRED_SCORE);
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
            if let Some(point) = game.valid_add_set(set) {
                game.add_set(set, point);
                if !gamestates[game.score() - 1]
                    .iter()
                    .any(|gs| v_hs_eq(gs, &game.sets))
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
    let mut sorted_possible_moves = Vec::with_capacity(possible_moves.len());
    for m in possible_moves.into_iter() {
        sorted_possible_moves.push(m);
    }
    // Sort by possible moves
    sorted_possible_moves.sort_unstable_by(cmp_quad_hpm);
    for (_, set, point) in sorted_possible_moves.into_iter() {
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
                if let Some(point) = game.valid_add_set(set) {
                    game.add_set(set, point);
                    if !gamestates[game.score() - 1]
                        .iter()
                        .any(|gs| v_hs_eq(gs, &game.sets))
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
    sorted_possible_moves.sort_unstable_by(cmp_quad_hpm);
    for (_, set, point) in sorted_possible_moves.into_iter() {
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

pub fn v_hs_eq(hs: &AHashSet<Set>, v: &Vec<Set>) -> bool {
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

pub fn base_highest_set_vec(gamestates: &mut Vec<Vec<Vec<Set>>>, mut game: Game, point: Point) -> Game {
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
                let mut present = game.sets.clone();
                present.sort_unstable_by(|s1, s2| s1.packed().cmp(&s2.packed()));
                if !gamestates[game.score() - 1]
                    .iter()
                    .any(|gs| v_eq(&present, gs))
                {
                    // print!("c");
                    gamestates[game.score() - 1].push(present);
                    let num_possible_moves = game.possible_moves();
                    possible_moves.insert((num_possible_moves, set, point));
                }
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
    sorted_possible_moves.sort_unstable_by(cmp_quad_hpm);
    for (_, set, point) in sorted_possible_moves.into_iter() {
        // Branch off into recursion-land for each possible move, and break out of the loop if one
        // of the branches meets the required number of moves. Otherwise, if the branch returns and
        // hasn't met the requested number of moves, undo the change made here and try again. We
        // also don't have to worry about filtering the moves we've collected here since that's
        // already been done in the possible move collection loop.
        game.add_set(set, point);
        if game.score() >= DESIRED_SCORE {
            break;
        }
        if branch_highest_set_vec(gamestates, &mut game) {
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
fn branch_highest_set_vec(gamestates: &mut Vec<Vec<Vec<Set>>>, game: &mut Game) -> bool {
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
                    let mut present = game.sets.clone();
                    present.sort_unstable_by(|s1, s2| s1.packed().cmp(&s2.packed()));
                    if !gamestates[game.score() - 1]
                        .iter()
                        .any(|gs| v_eq(&present, gs))
                    {
                        // print!("c");
                        gamestates[game.score() - 1].push(present);
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
    sorted_possible_moves.sort_unstable_by(cmp_quad_hpm);
    for (_, set, point) in sorted_possible_moves.into_iter() {
        game.add_set(set, point);
        if game.score() >= DESIRED_SCORE {
            return true;
        }
        if branch_highest_set_vec(gamestates, game) {
            return true;
        } else {
            game.remove_set(set, point);
        }
    }
    // Ran out of moves to try, and we didn't reach the requisite number, so return `false`.
    false
}

pub fn v_eq(v1: &Vec<Set>, v2: &Vec<Set>) -> bool {
    if v1.len() == v2.len() {
        for i in 0..v1.len() {
            if v1[i] != v2[i] {
                return false;
            }
        }
        true
    } else {
        false
    }
}
