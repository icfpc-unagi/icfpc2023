#![allow(non_snake_case)]
use clap::Parser;
use icfpc2023::{simple_hillclimb::simple_hillclimb, *};
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
    icfpc2023::differential::hillclimb_grad(&input, output, &args.save_dir, args.time_limit);
}
