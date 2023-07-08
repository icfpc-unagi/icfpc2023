#![allow(unused_imports)]
use std::{collections::BinaryHeap, net::SocketAddr};

use aead::NewAead;
use icfpc2023::{self, Input, read_input, P, mcf::weighted_matching, write_output, compute_score, compute_score_for_instruments, compute_score_for_a_musician_fast, compute_score_fast, candidate::get_candidate2};
use rand::Rng;

fn main() {

    let inp = read_input();

    let mut start = vec![];
    for i in 0..inp.pos.len() {
        start.push(2);
    }

    let mut best_score = 0;
    let mut best_ret = vec![];
    let mut best_start = start.clone();

    let mut rng = rand::thread_rng();

    loop {
        start = best_start.clone();
        let mut next_start = start.clone();
        for i in 0..inp.pos.len() {
            if rng.gen_range(0, 4) == 0{
                next_start[i] = rng.gen_range(0, 3);   
            }
            else{
                next_start[i] = best_start[i];
            }
        }

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
    
        dbg!(ans.0);
        dbg!(compute_score_fast(&inp, &ret).0);

        let mut cand2 = Vec::new();
        for i in 0..inp.musicians.len() {
            cand2.push(candidate[ans.1[i]]);
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

        let mut ret = Vec::new();
        for i in 0..inp.musicians.len() {
            ret.push(P(candidate[ans.1[i]].0, candidate[ans.1[i]].1));
        }

        let score = compute_score_fast(&inp, &ret).0;
        dbg!(score);
        if score > best_score{
            best_ret = ret.clone();
            best_score = score;
            best_start = next_start.clone();
            eprintln!("OK!");
        }
        write_output(&best_ret);

    }

    write_output(&best_ret);

    //dbg!(get_stage_diff(XY{x:inp.pos[0].0, y:inp.pos[0].1} , XY{x:inp.stage0.0, y:inp.stage0.1}, XY{x:inp.stage1.0, y:inp.stage1.1}));

}


