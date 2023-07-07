use anyhow::Result;
use tokio;

use icfpc2023::api;

#[tokio::main]
async fn main() -> Result<()> {
    let submissions = api::get_submissions(0, 1000).await?;
    serde_json::to_writer_pretty(std::io::stdout(), &submissions)?;
    Ok(())
}
