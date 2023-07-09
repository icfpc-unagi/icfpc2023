// Usage:
// $ cargo run --bin vis -- ./testdata/problems-42.json ./testdata/output-42.json --png output.png
#![allow(non_snake_case)]

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
    let input = read_input_from_file(&cli.input);
    let output = cli
        .output
        .map(|f| read_output_from_file(&f))
        .unwrap_or(vec![]);
    let color_type = cli.c.unwrap_or(1);
    let svg = vis::vis(&input, &output, color_type, cli.f.unwrap_or(!0), None);
    eprintln!("Score = {}", svg.0);
    println!("{}", svg.2);
    if let Some(png) = cli.png {
        let png_data = svg_to_png::svg_to_png(&svg.2.into()).unwrap();
        std::fs::write(png, png_data).unwrap();
    }
}
