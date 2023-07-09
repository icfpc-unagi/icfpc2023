#![allow(non_snake_case, unused_imports)]

use clap::Parser;
use icfpc2023::*;

#[derive(Parser, Debug)]
struct Args {
    /// Path to input.txt
    input: String,
    /// Path to output.txt
    output: String,
}

fn compute_max_distance(input: &Input, output: &Output) -> f64 {
    let mut d: f64 = 0.0;
    for p in &input.pos {
        for q in &output.0 {
            d = d.max((*p - *q).abs2().sqrt());
        }
    }
    d
}

fn compute_score_with_distance_limit(
    input: &Input,
    output: &Output,
    dist_limit: f64,
) -> (i64, i64) {
    if !is_valid_output(input, output, true) {
        return (0, 0);
    }

    let mut score = 0;
    let mut n_pairs = 0;
    for musician_id in 0..input.n_musicians() {
        for attendee_id in 0..input.n_attendees() {
            let d2 = (input.pos[attendee_id] - output.0[musician_id]).abs2();
            if d2 >= dist_limit * dist_limit {
                continue;
            }
            score += compute_score_for_pair(input, output, musician_id, attendee_id);
            n_pairs += 1;
        }
    }
    (score, n_pairs)
}

fn doit(input_path: &str, output_path: &str) {
    let input = read_input_from_file(input_path);
    let output = read_output_from_file(output_path);
    let true_score = compute_score(&input, &output);

    let max_dist = compute_max_distance(&input, &output);
    // dbg!(&max_dist);

    println!(
        "{:10}\t{:10}\t{:10}\t{}",
        "MaxDist", "#Pairs", "Score", "Error"
    );
    let mut dist_limit = 5.0;
    while dist_limit < max_dist {
        dist_limit *= 2.0;

        let (estimated_score, n_pairs) =
            compute_score_with_distance_limit(&input, &output, dist_limit);
        let error = (estimated_score - true_score) as f64 / (true_score as f64 + 1e-9);
        println!(
            "{:10}\t{:10}\t{:10}\t{:>+7.2}%",
            dist_limit,
            n_pairs,
            estimated_score,
            error * 100.0
        );
    }
}

fn main() {
    for i in 1..45 {
        println!("{}\nProblem {}", "-".repeat(80), i);
        doit(
            &format!("../Dropbox/ICFPC2023/problems/problem-{}.json", i),
            &format!("../Dropbox/ICFPC2023/chokudai-out1/{}.json", i),
        );
    }

    // let args = Args::parse();
    // dbg!(&args);
}
