use crate::DESIRED_SCORE;
use crate::game::{Game, STARTING_POINTS};
use crate::methods::{
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

fn multithreaded<MoveSet, MoveInfo, MoveIter>(
    method: Method<MoveSet, MoveInfo, MoveIter>
) -> Option<Game>
    where
        MoveInfo: Hash + Eq + PartialEq,
        MoveIter: Iterator<Item = (Set, Point)>
{
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
                        // base_highest_set(&mut t_gamestates,Game::new(), point),
                        // base_highest_set_vec(&mut t_gamestates, Game::new(), point),
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