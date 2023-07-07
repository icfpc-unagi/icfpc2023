use anyhow::Result;
use tokio;

use icfpc2023::api;

#[tokio::main]
async fn main() -> Result<()> {
    let submissions = api::submissions().await?;
    serde_json::to_writer_pretty(std::io::stdout(), &submissions)?;
    Ok(())
}
