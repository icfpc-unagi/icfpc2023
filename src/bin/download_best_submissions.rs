use std::{fs::File, path::Path};

use anyhow::Result;
use clap::Parser;
use icfpc2023::{api::*};
use std::io::*;

#[derive(Parser, Debug)]
struct Cli {
    /// Path to output directory.
    /// The best submission for each problem will be saved as {problem_id}-{submission_id}.json.
    #[clap(short, long, default_value = "best_submissions/")]
    output_dir: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let output_dir = Path::new(&args.output_dir);
    std::fs::create_dir_all(output_dir)?;
    download_all_best_submissions(output_dir).await?;
    Ok(())
}

/// Returns the best scored submission for every problem.
/// This function is very slow because it needs to fetch all submissions.
pub async fn download_all_best_submissions(output_dir: &Path) -> Result<()> {
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
            let output_path =
                output_dir.join(format!("{}-{}.json", submission.problem_id, submission._id));
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
    // let submission_ids = best_submissions
    //     .into_iter()
    //     .filter_map(|s| s.map(|s| s._id))
    //     .collect_vec();
    // let tasks = submission_ids
    //     .into_iter()
    //     .map(|s| {
    //         let output_path = output_dir.join(format!(
    //             "{}-{}.json",
    //             response.submission.problem_id, response.submission._id
    //         ));
    //         tokio::spawn(async move {
    //             match get_submission(&s).await {
    //                 Err(e) => eprintln!("Error getting submission {}: {}", s, e),
    //                 Ok(response) => {
    //                     let mut file = File::create(&output_path).unwrap();
    //                     file.write_all(response.contents.as_bytes()).unwrap();
    //                     eprintln!("Wrote {}", output_path.display());
    //                 }
    //             }
    //         })
    //     })
    //     .collect_vec();
    for task in tasks {
        task.await?;
    }
    Ok(())
}
