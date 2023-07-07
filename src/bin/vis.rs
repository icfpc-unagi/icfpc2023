#![allow(non_snake_case)]

use clap::Parser;
use icfpc2023::*;
use std::{io::prelude::*, path::PathBuf};

#[derive(Parser, Debug)]
struct Cli {
    /// Path to input.txt
    input: String,
    /// Path to output.txt
    output: String,
    /// color_type
    #[clap(long = "color_type")]
    c: Option<i32>,
}

fn main() {
    let cli = Cli::parse();
    let input = read_input_from_file(&cli.input);
    let output = read_output_from_file(&cli.output);
    let color_type = cli.c.unwrap_or(1);
    let svg = vis::vis(&input, &output, color_type);
    eprintln!("Score = {}", svg.0);
    println!("{}", svg.2);
}
