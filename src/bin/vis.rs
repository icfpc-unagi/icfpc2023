#![allow(non_snake_case)]

use clap::Parser;
use icfpc2023::*;

#[derive(Parser, Debug)]
struct Cli {
    /// Path to input.txt
    input: String,
    /// Path to output.txt
    output: Option<String>,
    /// color_type
    #[clap(long = "color_type")]
    c: Option<i32>,
    /// focus
    #[clap(long = "focus")]
    f: Option<usize>,
}

fn main() {
    let cli = Cli::parse();
    let input = read_input_from_file(&cli.input);
    let output = cli
        .output
        .map(|f| read_output_from_file(&f))
        .unwrap_or(vec![]);
    let color_type = cli.c.unwrap_or(1);
    let svg = vis::vis(&input, &output, color_type, cli.f.unwrap_or(!0));
    eprintln!("Score = {}", svg.0);
    println!("{}", svg.2);
}
