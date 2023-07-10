#![allow(non_snake_case)]
use clap::Parser;
use icfpc2023::*;
use rand::Rng;

#[derive(Parser, Debug)]
struct Args {
    input_path: String,
    output_path: String,
    save_dir: String,

    #[clap(long = "time-limit")]
    time_limit: Option<f64>,
}

fn main() {
    let args = Args::parse();
    let input = read_input_from_file(&args.input_path);
    let mut output = read_output_from_file(&args.output_path);
    let time_limit = args.time_limit.unwrap_or(1e9);

    let session_time_limit = 20.0;
    let time_start = get_time();
    let score_original = compute_score(&input, &output);

    loop {
        if get_time() - time_start > time_limit {
            break;
        }
        output = icfpc2023::differential::hillclimb_grad(
            &input,
            output,
            &args.save_dir,
            Some(session_time_limit),
        );

        if get_time() - time_start > time_limit {
            break;
        }
        output = simple_hillclimb::hillclimb_candidate_findbest(
            &input,
            output,
            &args.save_dir,
            1000,
            session_time_limit,
        );

        if get_time() - time_start > time_limit {
            break;
        }
        output = random_hillclimb::hillclimb_random_move(
            &input,
            output,
            &args.save_dir,
            Some(session_time_limit),
        );

        let score = compute_score(&input, &output);
        eprintln!(
            "UP-TOTAL {}\n{:10} -> {:10} ({:+10})\n{}",
            "=".repeat(80),
            score_original,
            score,
            score - score_original,
            "=".repeat(80)
        );
    }
}
