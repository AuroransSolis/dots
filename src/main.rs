extern crate ahash;
extern crate num_cpus;
extern crate rayon;

mod extras;
mod game;
mod methods;
mod point;
mod set;
mod solvers;

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
use solvers::multithreaded_method;

pub const DESIRED_SCORE: usize = 60;

fn main() {
    let method = Method::new(v_hs_eq, store_hs, store_npm_s_p, cmp_hpm, spm_intoiter_rev);
    let try_best = multithreaded_method(method);
    if let Some(best) = try_best {
        println!("Got: {}", best.score());
        for set in best.sets.iter() {
            println!("{}", set);
        }
    } else {
        println!(":c");
    }
}
