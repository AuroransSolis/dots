use crate::DESIRED_SCORE;
use crate::game::{Game, STARTING_POINTS};
use crate::methods::{
    base,
    Method
};
use crate::point::Point;
use crate::set::Set;
use rayon::prelude::*;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, TryRecvError};
use std::thread::spawn;

pub fn multithreaded_method<MoveSet: 'static, MoveInfo: 'static, MoveIter: 'static>(
    method: Method<MoveSet, MoveInfo, MoveIter>
) -> Option<Game>
    where
        MoveSet: Clone + Send,
        MoveInfo: Hash + Eq + PartialEq,
        MoveIter: Iterator<Item = (Set, Point)>
{
    let (send, recv) = channel();
    let starting_points = Arc::new(Mutex::new(STARTING_POINTS.iter().cloned()));
    for i in 0..num_cpus::get() {
        let mut t_gamestates: Vec<Vec<MoveSet>> = vec![Vec::new(); DESIRED_SCORE];
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
                        base(&mut t_gamestates, Game::new(), point, method),
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
    try_best
}

pub fn multithreaded<MoveSet: 'static + Clone + Send>(
    base: fn(&mut Vec<Vec<MoveSet>>, Game, Point) -> Game
) -> Option<Game> {
    let (send, recv) = channel();
    let starting_points = Arc::new(Mutex::new(STARTING_POINTS.iter().cloned()));
    for i in 0..num_cpus::get() {
        let mut t_gamestates: Vec<Vec<MoveSet>> = vec![Vec::new(); DESIRED_SCORE];
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
                        base(&mut t_gamestates, Game::new(), point),
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
    try_best
}

pub fn multithreaded_rayon_method<MoveSet: 'static, MoveInfo: 'static, MoveIter: 'static>(
    method: Method<MoveSet, MoveInfo, MoveIter>
) -> Option<Game>
    where
        MoveSet: Clone + Send,
        MoveInfo: Hash + Eq + PartialEq,
        MoveIter: Iterator<Item = (Set, Point)>
{
    STARTING_POINTS
        .par_iter()
        .map(|&point| base(&mut vec![Vec::new(); DESIRED_SCORE], Game::new(), point, method))
        .find_first(|game| game.score() >= DESIRED_SCORE)
}

pub fn multithreaded_rayon<MoveSet>(
    base: fn(&mut Vec<Vec<MoveSet>>, Game, Point) -> Game
) -> Option<Game>
    where MoveSet: Clone
{
    STARTING_POINTS
        .par_iter()
        .map(|&point| base(&mut vec![Vec::new(); DESIRED_SCORE], Game::new(), point))
        .find_first(|game| game.score() >= DESIRED_SCORE)
}

pub fn singlethreaded_method<MoveSet: 'static, MoveInfo: 'static, MoveIter: 'static>(
    method: Method<MoveSet, MoveInfo, MoveIter>
) -> Option<Game>
    where
        MoveSet: Clone + Send,
        MoveInfo: Hash + Eq + PartialEq,
        MoveIter: Iterator<Item = (Set, Point)>
{
    let mut gamestates = vec![Vec::new(); DESIRED_SCORE];
    STARTING_POINTS
        .iter()
        .map(|&point| base(&mut gamestates, Game::new(), point, method))
        .filter(|game| game.score() >= DESIRED_SCORE)
        .next()
}

pub fn singlethreaded<MoveSet: Clone>(
    base: fn(&mut Vec<Vec<MoveSet>>, Game, Point) -> Game
) -> Option<Game> {
    let mut gamestates = vec![Vec::new(); DESIRED_SCORE];
    STARTING_POINTS
        .iter()
        .map(|&point| base(&mut gamestates, Game::new(), point))
        .filter(|game| game.score() >= DESIRED_SCORE)
        .next()
}