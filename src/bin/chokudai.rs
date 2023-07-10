#![allow(unused_imports)]
use std::{cmp::max, collections::BinaryHeap, net::SocketAddr};

use aead::NewAead;
use icfpc2023::{
    self,
    candidate2::get_candidate3,
    compute_score, compute_score_fast, compute_score_for_a_musician_fast,
    compute_score_for_instruments, get_time,
    mcf::{weighted_matching, weighted_matching_with_capacity},
    read_input, write_output, Input, P,
};
use rand::Rng;

use clap::Parser;
use icfpc2023::*;

#[derive(Parser, Debug)]
struct Cli {
    /// Input path to input.txt
    input: String,
    /// Input path to output.txt
    output: Option<String>,
    /// color_type
    #[clap(long = "color_type")]
    c: Option<i32>,
    /// focus
    #[clap(long = "focus")]
    f: Option<usize>,
    // Output path to a png file to be saved.
    #[clap(long = "png")]
    png: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let inp = read_input_from_file(&cli.input);
    let output = cli
        .output
        .map(|f| read_output_from_file(&f))
        .unwrap_or((vec![], vec![]));

    //let inp = read_input();

    let mut best_ret = output.clone();
    let mut best_score = compute_score_fast(&inp, &best_ret).0;
    let mut best_cand: Vec<P> = best_ret.0.clone();
    let first_score = best_score;

    let mut rng = rand::thread_rng();

    let m = inp.musicians.len();

    let music_n = inp.tastes[0].len();
    let mut music_num = vec![0; music_n];
    let mut music_index = vec![vec![]; music_n];
    for i in 0..inp.musicians.len() {
        music_num[inp.musicians[i]] += 1;
        music_index[inp.musicians[i]].push(i);
    }
    let music_num = music_num;

    let tl: f64 = std::env::var("TL")
        .map(|a| a.parse().unwrap())
        .unwrap_or(300.0);
    let stime = get_time();

    let mut iter = 0;

    loop {
        let t = (get_time() - stime) / tl;
        if t >= 1.0 {
            //eprintln!("Iter = {}", iter);
            break;
        }

        iter += 1;

        let mut first_cand = vec![];
        if best_cand.len() != 0 {
            let target_cand = best_cand[rng.gen_range(0, best_cand.len())].clone();
            let target_range = rng.gen_range(30.0, 100.0);

            for i in 0..best_cand.len() {
                if (target_cand - best_cand[i]).abs() > target_range {
                    first_cand.push(best_cand[i].clone());
                }
            }
        }

        let candidate = get_candidate3(&inp, &first_cand, iter);

        let pos_to_music = compute_score_for_instruments(&inp, &candidate);

        //dbg!(candidate.len());
        let mut ar = Vec::new();
        for i in 0..pos_to_music[0].len() {
            let mut br = Vec::new();
            for j in 0..pos_to_music.len() {
                br.push(pos_to_music[j][i]);
            }
            ar.push(br);
        }

        let ans = weighted_matching_with_capacity(&ar, &music_num);

        let mut ret = (vec![P(0.0, 0.0); m], vec![10.0; m]);
        for i in 0..ans.1.len() {
            for j in 0..ans.1[i].len() {
                ret.0[music_index[i][j]] = candidate[ans.1[i][j]];
                if ar[i][ans.1[i][j]] == 0 {
                    ret.1[music_index[i][j]] = 0.0;
                }
            }
        }
        //let score = ans.0;

        let mut cand2 = Vec::new();
        for i in 0..inp.musicians.len() {
            cand2.push(ret.0[i]);
            //dbg!(pos_to_music[ans.1[i]][inp.musicians[i]]);
            //dbg!(compute_score_for_a_musician_fast(&inp, &ret, i).0);
        }

        let candidate = cand2.clone();
        let mut position = vec![];
        for i in 0..cand2.len() {
            position.push(cand2[i].clone());
        }

        let pos_to_music = compute_score_for_instruments(&inp, &position);

        let mut ar = Vec::new();
        for i in 0..pos_to_music[0].len() {
            let mut br = Vec::new();
            for j in 0..pos_to_music.len() {
                br.push(pos_to_music[j][i]);
            }
            ar.push(br);
        }

        let ans = weighted_matching_with_capacity(&ar, &music_num);

        let mut ret = (vec![P(0.0, 0.0); m], vec![10.0; m]);
        for i in 0..ans.1.len() {
            for j in 0..ans.1[i].len() {
                ret.0[music_index[i][j]] = cand2[ans.1[i][j]];
                if ar[i][ans.1[i][j]] == 0 {
                    ret.1[music_index[i][j]] = 0.0;
                }
            }
        }

        let score = compute_score_fast(&inp, &ret).0;

        //dbg!(score);
        if score > best_score {
            best_ret = ret.clone();
            best_score = score;
            best_cand = candidate.clone();
            eprintln!(
                "{} {} {} {} +{}",
                &cli.input,
                (get_time() - stime),
                iter,
                best_score,
                best_score - first_score
            );
        }
        //write_output(&best_ret);
    }

    write_output(&best_ret);

    //dbg!(get_stage_diff(XY{x:inp.pos[0].0, y:inp.pos[0].1} , XY{x:inp.stage0.0, y:inp.stage0.1}, XY{x:inp.stage1.0, y:inp.stage1.1}));
}
