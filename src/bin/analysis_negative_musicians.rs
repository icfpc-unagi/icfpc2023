#![allow(non_snake_case, unused_imports)]

use clap::Parser;
use icfpc2023::*;

fn main() {
    println!(
        "{:10}\t{:10}\t{:10}\t{:10}\t{:10}\t{:10}\t{:10}\t{:10}",
        "Problem", "Score", "TotalPos", "TotalNeg", "NPos", "NNeg", "AvgPos", "AvgNeg"
    );

    for i in 1..45 {
        // println!("{}\nProblem {}", "-".repeat(80), i);
        let input_path = format!("../Dropbox/ICFPC2023/problems/problem-{}.json", i);
        let output_path = format!("../Dropbox/ICFPC2023/chokudai-out1/{}.json", i);

        let input = read_input_from_file(&input_path);
        let output = read_output_from_file(&output_path);

        let mut total_score_pos = 0;
        let mut total_score_neg = 0;
        let mut n_pos_musicians = 0;
        let mut n_neg_musicians = 0;
        for musician_id in 0..input.n_musicians() {
            let mut musician_score = 0;
            for attendee_id in 0..input.n_attendees() {
                let score = compute_score_for_pair(&input, &output, musician_id, attendee_id);
                musician_score += score;
            }
            if musician_score > 0 {
                total_score_pos += musician_score;
                n_pos_musicians += 1;
            }
            if musician_score < 0 {
                total_score_neg += musician_score;
                n_neg_musicians += 1;
            }
        }

        println!(
            "{:10}\t{:10}\t{:10}\t{:10}\t{:10}\t{:10}\t{:10.1}\t{:10.1}",
            i,
            total_score_pos + total_score_neg,
            total_score_pos,
            total_score_neg,
            n_pos_musicians,
            n_neg_musicians,
            total_score_pos as f64 / n_pos_musicians as f64,
            total_score_neg as f64 / n_neg_musicians as f64,
        );
    }

    // let args = Args::parse();
    // dbg!(&args);
}
