use std::collections::HashSet;

use crate::*;

use actix_web::{HttpResponse, Responder};
use anyhow::Result;
use mysql::params;

async fn insert_official_submission(official_id: &str) -> Result<Option<String>> {
    let submission = api::get_submission_api(official_id).await?;
    eprintln!("Updating submission: {:#?}", &submission);
    let (submission_score, submission_error) = match submission.submission.score {
        api::SubmissionStatus::Processing => (None, None),
        api::SubmissionStatus::Success(score) => (Some(score), None),
        api::SubmissionStatus::Failure(error) => (None, Some(error)),
    };
    sql::exec(
        "REPLACE INTO submissions(
        official_id,
        problem_id,
        submission_score,
        submission_error,
        submission_contents,
        submission_created
    ) VALUES (
        :official_id,
        :problem_id,
        :submission_score,
        :submission_error,
        :submission_contents,
        :submission_created
    )",
        params! {
            "official_id" => &submission.submission._id,
            "problem_id" => submission.submission.problem_id,
            "submission_score" => submission_score,
            "submission_error" => submission_error,
            "submission_contents" => submission.contents,
            "submission_created" => submission.submission.submitted_at.replace("T", " ").split(".").next().unwrap(),
        },
    )?;
    Ok(Some(submission.submission._id))
}

pub async fn update_official_submissions(offset: u32, limit: u32) -> Result<Vec<String>> {
    let submissions = api::get_submissions(offset, limit).await?;
    let mut ids = Vec::new();
    for submission in submissions {
        eprintln!("Checking submission: {}", submission._id);
        let result = sql::row(
            "
            SELECT submission_id, submission_score, submission_error
            FROM submissions
            WHERE official_id = :official_id",
            params! {
                "official_id" => &submission._id,
            },
        )?;
        let (submission_score, submission_error) = match submission.score {
            api::SubmissionStatus::Processing => (None, None),
            api::SubmissionStatus::Success(score) => (Some(score), None),
            api::SubmissionStatus::Failure(error) => (None, Some(error)),
        };
        let should_update = match result {
            Some(row) => {
                let db_submission_score: Option<u64> = row.get_option("submission_score")?;
                let db_submission_error: Option<String> = row.get_option("submission_error")?;
                submission_score != db_submission_score || submission_error != db_submission_error
            }
            None => true,
        };
        if should_update {
            if let Some(id) = insert_official_submission(&submission._id).await? {
                ids.push(id);
            }
        }
    }
    Ok(ids)
}

pub async fn update_official_problem(problem_id: u32) -> Result<()> {
    const CHUNK_SIZE: usize = 1024 * 1024;
    const TEMP_ID: u32 = 100000;

    eprintln!("Updating the problem: {}", problem_id);
    let problem = api::get_raw_problem(problem_id).await?;
    dbg!(&problem);
    sql::exec(
        "
        DELETE FROM problem_chunks
        WHERE problem_id = :problem_id",
        params! {
            "problem_id" => problem_id + TEMP_ID,
            "temp_id" => &TEMP_ID,
        },
    )?;
    for (chunk_index, chunk) in problem
        .as_str()
        .chars()
        .collect::<Vec<char>>()
        .chunks(CHUNK_SIZE)
        .map(|chunk| chunk.into_iter().collect::<String>())
        .enumerate()
    {
        sql::exec(
            "
            INSERT INTO problem_chunks(
                problem_id,
                problem_chunk_index,
                problem_chunk,
                problem_chunk_checked
            ) VALUES (
                :problem_id,
                :problem_chunk_index,
                :problem_chunk,
                CURRENT_TIMESTAMP()
            )",
            params! {
                "problem_id" => problem_id + TEMP_ID,
                "problem_chunk_index" => chunk_index,
                "problem_chunk" => &chunk,
            },
        )?;
    }
    sql::exec(
        "
        UPDATE problem_chunks
        SET problem_id = problem_id - :temp_id
        WHERE problem_id = :problem_id OR problem_id = :problem_id + :temp_id",
        params! {
            "problem_id" => problem_id + TEMP_ID,
            "temp_id" => &TEMP_ID,
        },
    )?;
    sql::exec(
        "
        DELETE FROM problem_chunks
        WHERE problem_id < 0",
        mysql::Params::Empty,
    )?;
    Ok(())
}

pub async fn update_official_problems() -> Result<Vec<u32>> {
    let mut problem_ids = HashSet::<u32>::new();
    for row in sql::select(
        "
        SELECT DISTINCT problem_id FROM problem_chunks
    ",
        mysql::Params::Empty,
    )? {
        problem_ids.insert(row.get("problem_id")?);
    }

    let mut updated_ids = Vec::new();
    let num_problems = api::get_number_of_problems().await?;
    for problem_id in 1..=num_problems {
        if !problem_ids.contains(&problem_id) {
            update_official_problem(problem_id).await?;
            updated_ids.push(problem_id);
        }
    }
    Ok(updated_ids)
}

pub mod update_problem_png {
    use super::*;

    use crate::api;

    pub async fn update(problem_id: u32) -> Result<()> {
        eprintln!("Updating the problem png: {}", problem_id);
        let problem = api::get_problem(problem_id).await?;
        let svg = vis::vis(&problem.into(), &Vec::new(), 1, !0, None);
        let png_data = svg_to_png::svg_to_png(&svg.2.into())?;
        sql::exec(
            "
INSERT INTO problem_pngs(
    problem_id,
    problem_png_data
) VALUES (
    :problem_id,
    :problem_png_data
) ON DUPLICATE KEY UPDATE
    problem_png_data = :problem_png_data",
            params! {
                "problem_id" => problem_id,
                "problem_png_data" => &png_data,
            },
        )
    }

    pub async fn update_all() -> Result<Vec<u32>> {
        let mut problem_ids = Vec::new();
        for row in sql::select(
            "SELECT DISTINCT problem_id FROM problem_chunks",
            mysql::Params::Empty,
        )? {
            let problem_id = row.get("problem_id")?;
            update(problem_id).await?;
            problem_ids.push(problem_id);
        }
        Ok(problem_ids)
    }
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
    match update_official_problems().await {
        Ok(ids) => {
            if ids.len() == 0 {
                buf.push_str("No problems to update\n");
            } else {
                buf.push_str(&format!("Added problems: {:?}\n", ids));
            }
        }
        Err(e) => {
            buf.push_str(&format!("Failed to update problems: {}\n", e));
        }
    }
    HttpResponse::Ok().content_type("text/plain").body(buf)
}
