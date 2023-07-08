use crate::*;

use actix_web::{HttpResponse, Responder};
use anyhow::Result;
use mysql::params;

async fn insert_official_submission(official_id: &str) -> Result<Option<String>> {
    let submission = api::get_submission(official_id).await?;
    dbg!(&submission);
    let (submission_score, submission_error) = match submission.submission.score {
        api::SubmissionStatus::Processing => {
            return Ok(None);
        }
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
        "official_id" => &submission.submission._id,
        "problem_id" => submission.submission.problem_id,
        "submission_score" => submission_score,
        "submission_error" => submission_error,
        "submission_contents" => submission.contents,
        "submission_crated" => submission.submission.submitted_at.replace("T", " ").split(".").next().unwrap(),
    })?;
    Ok(Some(submission.submission._id))
}

pub async fn update_official_submissions(offset: u32, limit: u32) -> Result<Vec<String>> {
    let submissions = api::get_submissions(offset, limit).await?;
    let mut ids = Vec::new();
    for submission in submissions {
        dbg!(&submission);
        let result: Option<u32> = sql::cell(
            "
            SELECT submission_id
            FROM submissions
            WHERE official_id = :official_id",
            params! {
                "official_id" => &submission._id,
            },
        )?;
        if let None = result {
            if let Some(id) = insert_official_submission(&submission._id).await? {
                ids.push(id);
            }
        }
    }
    Ok(ids)
}

pub async fn handler() -> impl Responder {
    let mut buf = String::new();
    match update_official_submissions(0, 100).await {
        Ok(ids) => {
            if ids.len() == 0 {
                buf.push_str("No submissions to update\n");
            } else {
                buf.push_str(&format!("Added submissions: {:?}\n", ids));
            }
        }
        Err(e) => {
            buf.push_str(&format!("Failed to update submissions: {}\n", e));
        }
    }
    HttpResponse::Ok().content_type("text/plain").body(buf)
}
