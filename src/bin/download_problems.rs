use anyhow::{anyhow, Result};
use clap::Parser;
use icfpc2023::api;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
struct Cli {
    /// Path to output directory
    #[arg(short = 'o', long, default_value = "problems/")]
    output_dir: PathBuf,

    #[arg(short = 'w', long)]
    overwrite: bool,
}

/// Downloads all problems and writes them to files under problems/.
#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let number_of_problems = api::get_number_of_problems().await?;
    eprintln!("There are {} problems found in total.", number_of_problems);

    for problem_id in 1..=number_of_problems {
        let output_path = args.output_dir.join(format!("problem-{}.json", problem_id));
        match download_and_write_problem(&output_path, problem_id, args.overwrite).await {
            Ok(_) => {
                eprintln!(
                    "Successfully downloaded and wrote data for problem_id={} to {}",
                    problem_id,
                    output_path.display(),
                )
            }
            Err(error) => {
                eprintln!("problem_id={}: {}", problem_id, error);
            }
        }
    }

    Ok(())
}

async fn download_and_write_problem(
    output_path: &Path,
    problem_id: u32,
    overwrite: bool,
) -> Result<()> {
    // Skip if the file already exists.
    if !overwrite && output_path.exists() {
        return Err(anyhow!(
            "File for problem_id={} ({}) already exists. Skipping...",
            problem_id,
            output_path.display(),
        ));
    }

    // Download the problem.
    let raw_problem = api::get_raw_problem(problem_id).await?;

    // Canonicalize the JSON.
    let parsed: serde_json::Value = serde_json::from_str(&raw_problem)?;
    let canonicalized = parsed.to_string();

    // Save the problem to a file.
    let mut file = File::create(&output_path)?;
    file.write_all(canonicalized.as_bytes())?;

    Ok(())
}
