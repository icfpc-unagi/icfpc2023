use clap::Parser;
use icfpc2023::*;

#[derive(Parser, Debug)]
struct Cli {
    /// Path to input.txt
    input: String,
    /// Path to output.txt
    output: String,
}

fn main() {
    let cli = Cli::parse();
    let input = read_input_from_file(&cli.input);
    let output = read_output_from_file(&cli.output);
    let score = compute_score(&input, &output);
    let bigint_score = bigint_scoring::compute_score(&input, &output);
    println!("score = {score}, bigint_score = {bigint_score}");
    assert_eq!(score, bigint_score);
}
