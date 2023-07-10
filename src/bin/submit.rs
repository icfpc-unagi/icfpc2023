use anyhow::Result;
use clap::Parser;
use icfpc2023::api;
use icfpc2023::*;

#[derive(Parser)]
struct Cli {
    problem_id: u32,
    /// Path to the output json file.
    solution: String,
    /// Tags to be added to the submission.
    #[arg(short, long)]
    tags: Vec<String>,
    /// Whether to submit to the local server.
    #[arg(short, long, default_value_t = true)]
    local: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let solution = read_output_from_file(&args.solution);
    let tags: Vec<&str> = args.tags.iter().map(|s| s.as_str()).collect();
    let submission_id = api::submit(args.problem_id, &solution, &tags, args.local).await?;

    eprintln!("Submitted as submission_id={}", submission_id);

    Ok(())
}
