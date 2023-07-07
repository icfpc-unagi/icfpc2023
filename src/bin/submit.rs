use anyhow::Result;
use clap::Parser;
use icfpc2023::api;
use icfpc2023::*;

#[derive(Parser, Debug)]
struct Cli {
    problem_id: u32,
    /// Path to the output json file.
    solution: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let solution = read_output_from_file(&args.solution);
    let submission_id = api::submit(args.problem_id, &solution).await?;

    eprintln!("Submitted as submission_id={}", submission_id);

    Ok(())
}
