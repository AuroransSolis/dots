extern crate ahash;
extern crate num_cpus;

mod extras;
mod game;
mod methods;
mod point;
mod set;

use game::{Game, STARTING_POINTS};
use methods::{
    base,
    base_highest_set,
    base_highest_set_vec,
    cmp_hpm,
    cmp_mnx_hpm,
    cmp_quad_hpm,
    Method,
    spm_intoiter,
    spm_intoiter_rev,
    store_hs,
    store_npm_s_p,
    store_v,
    v_eq,
    v_hs_eq
};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, TryRecvError};
use std::thread::spawn;

pub const DESIRED_SCORE: usize = 60;

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
    let method = Method::new(v_hs_eq, store_hs, store_npm_s_p, cmp_hpm, spm_intoiter_rev);
    let mut gamestates = vec![Vec::new(); DESIRED_SCORE];
    let try_best = STARTING_POINTS
        .iter().take(1)
        .map(|&point| base(&mut gamestates, Game::new(), point, method))
        .next();
    /*let try_best = STARTING_POINTS
        .par_iter()
        .map(|&point| base_highest_set(gamestates.clone(), Game::new(), point))
        .find_first(|game| game.score() >= DESIRED_SCORE);*/
    /*let (send, recv) = channel();
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
                        // base_highest_set(&mut t_gamestates,Game::new(), point),
                        base_highest_set_vec(&mut t_gamestates, Game::new(), point),
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
    }*/
    if let Some(best) = try_best {
        println!("Got: {}", best.score());
        for set in best.sets.iter() {
            println!("{}", set);
        }
    } else {
        println!(":c");
    }
}
