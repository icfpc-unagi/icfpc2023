#![allow(unused_imports)]
use std::{collections::BinaryHeap, net::SocketAddr};

use aead::NewAead;
use icfpc2023::{
    self, candidate::get_candidate2, compute_score, compute_score_fast,
    compute_score_for_a_musician_fast, compute_score_for_instruments, get_time,
    mcf::weighted_matching, read_input, write_output, Input, P,
};
use rand::Rng;

fn main() {
    let inp = read_input();

    let mut start = vec![];
    for _i in 0..inp.pos.len() {
        start.push(1);
    }

    let mut best_score = 0;
    let mut best_ret = (vec![], vec![]);
    let mut best_start = start.clone();

    let mut rng = rand::thread_rng();

    let tl: f64 = std::env::var("TL")
        .map(|a| a.parse().unwrap())
        .unwrap_or(600.0);
    let stime = get_time();

    let mut iter = 0;

    loop {
        let t = (get_time() - stime) / tl;
        if t >= 1.0 {
            eprintln!("Iter = {}", iter);
            break;
        }

        start = best_start.clone();
        let mut next_start = start.clone();
        let mut chflag = false;

        if iter != 0 {
            for i in 0..inp.pos.len() {
                if rng.gen_range(0, inp.pos.len()) <= 100 {
                    next_start[i] = rng.gen_range(0, 3);
                    chflag = true;
                } else {
                    next_start[i] = best_start[i];
                }
            }
        } else {
            chflag = true;
        }
        if !chflag {
            continue;
        }
        iter += 1;

        let candidate = get_candidate2(&inp, &next_start);

        let pos_to_music = compute_score_for_instruments(&inp, &candidate);

        dbg!(candidate.len());
        let mut ar = Vec::new();
        for i in 0..inp.musicians.len() {
            let mut br = Vec::new();
            for j in 0..candidate.len() {
                br.push(pos_to_music[j][inp.musicians[i]]);
            }
            ar.push(br);
        }

        let ans = weighted_matching(&ar);
        let mut ret = Vec::new();
        for i in 0..inp.musicians.len() {
            ret.push(P(candidate[ans.1[i]].0, candidate[ans.1[i]].1));
        }

        //let score = ans.0;

        let mut cand2 = Vec::new();
        for i in 0..inp.musicians.len() {
            cand2.push(ret[i]);
            //dbg!(pos_to_music[ans.1[i]][inp.musicians[i]]);
            //dbg!(compute_score_for_a_musician_fast(&inp, &ret, i).0);
        }
        let candidate = cand2;

        let pos_to_music = compute_score_for_instruments(&inp, &candidate);

        let mut ar = Vec::new();
        for i in 0..inp.musicians.len() {
            let mut br = Vec::new();
            for j in 0..candidate.len() {
                br.push(pos_to_music[j][inp.musicians[i]]);
            }
            ar.push(br);
        }

        let ans = weighted_matching(&ar);

        let mut ret = (vec![], vec![]);
        for i in 0..inp.musicians.len() {
            ret.0.push(P(candidate[ans.1[i]].0, candidate[ans.1[i]].1));
            ret.1.push(1.0); // default volume
        }

        let score = compute_score_fast(&inp, &ret).0;

        dbg!(score);
        if score > best_score {
            best_ret = ret.clone();
            best_score = score;
            best_start = next_start.clone();
            eprintln!("OK!");
        }
        //write_output(&best_ret);
    }

    {
        //dbg!(ans.0);
        //dbg!(compute_score_fast(&inp, &ret).0);

        let mut cand2 = Vec::new();
        for i in 0..inp.musicians.len() {
            cand2.push((best_ret.0[i], best_ret.1[i]));
            //dbg!(pos_to_music[ans.1[i]][inp.musicians[i]]);
            //dbg!(compute_score_for_a_musician_fast(&inp, &ret, i).0);
        }
        let candidate = cand2;

        let positions = Vec::from_iter(candidate.iter().map(|a| a.0));
        let pos_to_music = compute_score_for_instruments(&inp, &positions);

        let mut ar = Vec::new();
        for i in 0..inp.musicians.len() {
            let mut br = Vec::new();
            for j in 0..candidate.len() {
                br.push(pos_to_music[j][inp.musicians[i]]);
            }
            ar.push(br);
        }

        let ans = weighted_matching(&ar);

        let mut ret = (vec![], vec![]);
        for i in 0..inp.musicians.len() {
            ret.0
                .push(P(candidate[ans.1[i]].0 .0, candidate[ans.1[i]].0 .1));
            ret.1.push(candidate[ans.1[i]].1)
        }

        let score = compute_score_fast(&inp, &ret).0;

        best_ret = ret;

        dbg!(score);
    }

    write_output(&best_ret);

    //dbg!(get_stage_diff(XY{x:inp.pos[0].0, y:inp.pos[0].1} , XY{x:inp.stage0.0, y:inp.stage0.1}, XY{x:inp.stage1.0, y:inp.stage1.1}));
}
