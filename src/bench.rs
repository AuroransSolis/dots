#[macro_use] extern crate criterion;

mod extras;
mod game;
mod methods;
mod point;
mod set;

use ahash::AHashSet;
use criterion::{black_box, Criterion};
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
use point::Point;
use set::{Direction, Set};

pub const DESIRED_SCORE: usize = 45;

const POINT: Point = STARTING_POINTS[0];

fn bench_method_set_hpm(c: &mut Criterion) {
    c.bench_function(format!("[{}] method (set hpm)", DESIRED_SCORE).as_str(), move |b| {
        let method = Method::new(v_hs_eq, store_hs, store_npm_s_p, cmp_hpm, spm_intoiter_rev);
        b.iter_with_setup(|| (Game::new(), vec![Vec::new(); DESIRED_SCORE]), |(g, mut gs)| {
            black_box(base(&mut gs, g, POINT, method));
        })
    });
}

fn bench_method_set_mnx_hpm(c: &mut Criterion) {
    c.bench_function(format!("[{}] method (set mnx -> hpm)", DESIRED_SCORE).as_str(), move |b| {
        let method = Method::new(v_hs_eq, store_hs, store_npm_s_p, cmp_mnx_hpm, spm_intoiter_rev);
        b.iter_with_setup(|| (Game::new(), vec![Vec::new(); DESIRED_SCORE]), |(g, mut gs)| {
            black_box(base(&mut gs, g, POINT, method));
        })
    });
}

fn bench_method_set_quad_hpm(c: &mut Criterion) {
    c.bench_function(format!("[{}] method (set quad -> hpm)", DESIRED_SCORE).as_str(), move |b| {
        let method = Method::new(v_hs_eq, store_hs, store_npm_s_p, cmp_quad_hpm, spm_intoiter);
        b.iter_with_setup(|| (Game::new(), vec![Vec::new(); DESIRED_SCORE]), |(g, mut gs)| {
            black_box(base(&mut gs, g, POINT, method));
        })
    });
}

fn bench_method_vec_hpm(c: &mut Criterion) {
    c.bench_function(format!("[{}] method (vec hpm)", DESIRED_SCORE).as_str(), move |b| {
        let method = Method::new(v_eq, store_v, store_npm_s_p, cmp_hpm, spm_intoiter_rev);
        b.iter_with_setup(|| (Game::new(), vec![Vec::new(); DESIRED_SCORE]), |(g, mut gs)| {
            black_box(base(&mut gs, g, POINT, method));
        })
    });
}

fn bench_method_vec_mnx_hpm(c: &mut Criterion) {
    c.bench_function(format!("[{}] method (vec mnx -> hpm)", DESIRED_SCORE).as_str(), move |b| {
        let method = Method::new(v_eq, store_v, store_npm_s_p, cmp_mnx_hpm, spm_intoiter_rev);
        b.iter_with_setup(|| (Game::new(), vec![Vec::new(); DESIRED_SCORE]), |(g, mut gs)| {
            black_box(base(&mut gs, g, POINT, method));
        })
    });
}

fn bench_method_vec_quad_hpm(c: &mut Criterion) {
    c.bench_function(format!("[{}] method (vec quad -> hpm)", DESIRED_SCORE).as_str(), move |b| {
        let method = Method::new(v_eq, store_v, store_npm_s_p, cmp_quad_hpm, spm_intoiter);
        b.iter_with_setup(|| (Game::new(), vec![Vec::new(); DESIRED_SCORE]), |(g, mut gs)| {
            black_box(base(&mut gs, g, POINT, method));
        })
    });
}

criterion_group! {
    name = bench;
    config = Criterion::default();
    targets = bench_method_set_hpm, bench_method_set_mnx_hpm, bench_method_set_quad_hpm,
        bench_method_vec_hpm, bench_method_vec_mnx_hpm, bench_method_vec_quad_hpm
}

criterion_main!{bench}