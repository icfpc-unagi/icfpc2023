use std::{fs::File, path::Path};

use anyhow::Result;
use clap::Parser;
use icfpc2023::{api::*, sql};
use std::io::*;

#[derive(Parser, Debug)]
struct Cli {
    /// Path to output directory.
    /// The best submission for each problem will be saved as {problem_id}-{score}-{submission_id}.json.
    #[clap(short, long, default_value = "best_submissions/")]
    output_dir: String,

    /// Use local database instead of API and count local only submissions as well.
    #[arg(short, long, default_value_t = true)]
    local: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let output_dir = Path::new(&args.output_dir);
    std::fs::create_dir_all(output_dir)?;
    if args.local {
        download_all_best_submissions_db(output_dir).await?;
    } else {
        download_all_best_submissions_api(output_dir).await?;
    }
    Ok(())
}

async fn download_all_best_submissions_db(output_dir: &Path) -> Result<()> {
    // Query all columns of the maximum submission_score for each problem_id,
    // but emit only one row for each problem_id.
    let rows = sql::select(
        r#"SELECT 
        s1.submission_id, 
        s1.official_id, 
        s1.problem_id, 
        s1.submission_score, 
        s1.submission_error, 
        s1.submission_contents, 
        s1.submission_created
      FROM submissions s1
      WHERE s1.submission_id = (
        SELECT s2.submission_id 
        FROM submissions s2 
        WHERE s2.problem_id = s1.problem_id 
        ORDER BY s2.submission_score DESC 
        LIMIT 1
      )"#,
        mysql::Params::Empty,
    )?;
    for row in rows {
        let submission_id = row.get::<u32>("submission_id")?;
        let official_id = row.get::<Option<String>>("official_id")?;
        let problem_id = row.get::<u32>("problem_id")?;
        let submission_score = row.get::<i64>("submission_score")?;
        let submission_error = row.get_option::<String>("submission_error")?;
        let submission_contents = row.get::<String>("submission_contents")?;
        // let _submission_created = row.get::<String>("submission_created")?;
        if let Some(error) = submission_error {
            eprintln!(
                "invalid submission found for some reason:\n submission_id: {}, submission_error: {}",
                submission_id, error
            );
            continue;
        }
        let output_path = output_dir.join(format!(
            "{}-{}-{}.json",
            problem_id,
            submission_score,
            official_id.unwrap_or(submission_id.to_string())
        ));
        let mut file = File::create(&output_path)?;
        file.write_all(submission_contents.as_bytes()).unwrap();
        eprintln!("Wrote {}", output_path.display());
    }
    Ok(())
}

/// Returns the best scored submission for every problem.
/// This function is very slow because it needs to fetch all submissions.
async fn download_all_best_submissions_api(output_dir: &Path) -> Result<()> {
    let n = get_number_of_problems().await?;
    let limit = 1000;
    // indexed by problem_id-1
    let mut best_submissions: Vec<Option<Submission>> = vec![None; n as usize];
    for offset in (0..).step_by(limit as usize) {
        let submissions = get_submissions(offset, limit).await?;
        let end = submissions.len() < limit as usize;
        for submission in submissions {
            eprintln!("checking submission: {:?}", submission);
            if let SubmissionStatus::Success(score) = &submission.score {
                let i = submission.problem_id as usize - 1;
                if match &best_submissions[i] {
                    Some(best_submission) => {
                        if let SubmissionStatus::Success(best_score) = best_submission.score {
                            best_score < *score
                        } else {
                            true
                        }
                    }
                    None => true,
                } {
                    best_submissions[i] = Some(submission);
                }
            }
        }
        if end {
            break;
        }
    }

    let mut tasks = Vec::new();
    for submission in best_submissions {
        if let Some(submission) = submission {
            let output_path = output_dir.join(format!(
                "{}-{}-{}.json",
                submission.problem_id, submission.score, submission._id
            ));
            let mut file = File::create(&output_path)?;
            tasks.push(tokio::spawn(async move {
                match get_submission(&submission._id).await {
                    Err(e) => eprintln!("Error getting submission {}: {}", submission._id, e),
                    Ok(response) => {
                        file.write_all(response.contents.as_bytes()).unwrap();
                        eprintln!("Wrote {}", output_path.display());
                    }
                }
            }));
        }
    }
    for task in tasks {
        task.await?;
    }
    Ok(())
}
