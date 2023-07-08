#![allow(unused_imports)]
use icfpc2023::{api, sql};
use mysql::params;
use anyhow::Result;

async fn insert_official_submission(official_id: &str) -> Result<()> {
    let submission = api::get_submission(official_id).await?;
    dbg!(&submission);
    let (submission_score, submission_error) = match submission.submission.score {
        api::SubmissionStatus::Processing => { return Ok(()); },
        api::SubmissionStatus::Success(score) => (Some(score), None),
        api::SubmissionStatus::Failure(error) => (None, Some(error)),
    };
    sql::exec("INSERT INTO submissions(
        official_id,
        problem_id,
        submission_score,
        submission_error,
        submission_contents,
        submission_created
    ) VALUES (:official_id, :problem_id, :submission_score, :submission_error, :submission_contents, :submission_crated)", params! {
        "official_id" => submission.submission._id,
        "problem_id" => submission.submission.problem_id,
        "submission_score" => submission_score,
        "submission_error" => submission_error,
        "submission_contents" => submission.contents,
        "submission_crated" => submission.submission.submitted_at.replace("T", " ").split(".").next().unwrap(),
    })?;
    Ok(())
}

async fn update_official_submissions(offset: u32, limit: u32) -> Result<()> {
    let submissions = api::get_submissions(offset, limit).await?;
    for submission in submissions {
        dbg!(&submission);
        let result: Option<u32> = sql::cell("
            SELECT submission_id
            FROM submissions
            WHERE official_id = :official_id", params! {
            "official_id" => &submission._id,
        })?;
        if let None = result {
            insert_official_submission(&submission._id).await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    for i in 0..10 {
        update_official_submissions(i * 100, 100).await.unwrap();
    }
}
