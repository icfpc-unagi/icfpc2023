use anyhow::Result;
use clap::Parser;
use tokio;

use icfpc2023::api;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(default_value_t = 0)]
    offset: u32,
    #[arg(default_value_t = 1000)]
    limit: u32,
}

/// Print a number of submissions sorted by submission time most recent first.
#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let submissions = api::get_submissions(args.offset, args.limit).await?;
    serde_json::to_writer_pretty(std::io::stdout(), &submissions)?;
    Ok(())
}
