use anyhow::Result;
use clap::Parser;
use tokio;

use icfpc2023::api;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(default_value_t = 0)]
    problem_id: u32,
}

/// Print a number of submissions sorted by submission time most recent first.
#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let problem = api::get_raw_problem_db(args.problem_id).await?;
    println!("{}", problem);
    Ok(())
}
