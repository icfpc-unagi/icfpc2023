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
        .unwrap_or(600.0);
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

            let maxdiff = (inp.stage1.0 - inp.stage0.0 + inp.stage1.1 - inp.stage0.1) / 4.0;
            let target_range = rng.gen_range(20.0, maxdiff);
            //let target_range = 50000.0;

            for i in 0..best_cand.len() {
                if (target_cand - best_cand[i]).abs() > target_range {
                    first_cand.push(best_cand[i].clone());
                }
            }
        }

        let candidate = get_candidate3(&inp, &first_cand, iter, inp.pillars.len() != 0);

        if inp.pillars.len() == 0 {
            let pos_to_music = compute_score_for_instruments(&inp, &candidate);

            //dbg!(candidate.len());
            let mut ar = Vec::new();
            for i in 0..pos_to_music[0].len() {
                let mut br = Vec::new();
                for j in 0..pos_to_music.len() {
                    if pos_to_music[j][i] > 0 {
                        br.push(pos_to_music[j][i]);
                    } else {
                        br.push(0);
                    }
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
                    if pos_to_music[j][i] > 0 {
                        br.push(pos_to_music[j][i]);
                    } else {
                        br.push(0);
                    }
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
            if score > best_score || true {
                best_ret = ret.clone();
                let diff = score - best_score;
                best_score = score;
                best_cand = candidate.clone();
                eprintln!(
                    "{} {} +{} (first +{}) {} {}",
                    &cli.input,
                    best_score,
                    diff,
                    best_score - first_score,
                    (get_time() - stime),
                    iter,
                );
            }
            break;
        } else {
            let mut ret = best_ret.clone();
            let mut pre_score = -999999999;

            let pos_to_music = compute_score_for_instruments(&inp, &candidate);

            let mut ar = Vec::new();
            for i in 0..pos_to_music[0].len() {
                let mut br = Vec::new();
                for j in 0..pos_to_music.len() {
                    if pos_to_music[j][i] > 0 {
                        br.push(pos_to_music[j][i]);
                    } else {
                        br.push(0);
                    }
                }
                ar.push(br);
            }

            for ttt in 0..5 {
                let ret_to_music = compute_score_for_instruments(&inp, &(ret.0));

                let mut effect = vec![vec![0; candidate.len()]; music_n];

                for i in 0..ret.0.len() {
                    let target_p = ret.0[i];
                    let music = inp.musicians[i];
                    let attack = ret_to_music[i][music];

                    for j in 0..candidate.len() {
                        let mut di = (candidate[j] - target_p).abs();
                        if di < 10.0 {
                            di = 10.0;
                            //continue;
                            //di = 1.0;
                        }
                        if ttt != 0 {
                            effect[music][j] += ((attack as f64) * (1.0 / di)) as i64;
                        }
                    }
                }

                let mut cr = vec![vec![0; candidate.len()]; music_n];

                for i in 0..music_n {
                    for j in 0..candidate.len() {
                        cr[i][j] = ar[i][j] + effect[i][j];
                        //dbg!(cr[i][j]);
                    }
                }

                //eprintln!("{} {} {} {}", ar.len(), ar[0].len(), cr.len(), cr[0].len());
                let ans = weighted_matching_with_capacity(&cr, &music_num);

                //let mut myans = 0;
                for i in 0..ans.1.len() {
                    for j in 0..ans.1[i].len() {
                        ret.0[music_index[i][j]] = candidate[ans.1[i][j]];
                        if cr[i][ans.1[i][j]] <= 0 {
                            ret.1[music_index[i][j]] = 0.0;
                        } else {
                            ret.1[music_index[i][j]] = 10.0;
                            //myans += cr[i][ans.1[i][j]] * 10;
                        }
                    }
                }

                let score = compute_score_fast(&inp, &ret).0;
                //dbg!(score);

                if pre_score == score {
                    break;
                }
                pre_score = score;
            }

            if pre_score > best_score {
                best_ret = ret.clone();
                let diff = pre_score - best_score;
                best_score = pre_score;
                best_cand = candidate.clone();
                eprintln!(
                    "{} {} +{} (first +{}) {} {}",
                    &cli.input,
                    best_score,
                    diff,
                    best_score - first_score,
                    (get_time() - stime),
                    iter,
                );
            }
        }
        //write_output(&best_ret);
    }

    write_output(&best_ret);

    //dbg!(get_stage_diff(XY{x:inp.pos[0].0, y:inp.pos[0].1} , XY{x:inp.stage0.0, y:inp.stage0.1}, XY{x:inp.stage1.0, y:inp.stage1.1}));
}
