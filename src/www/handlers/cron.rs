use std::collections::HashSet;

use crate::*;

use actix_web::{HttpResponse, Responder};
use anyhow::Result;
use mysql::params;
use rand::seq::SliceRandom;

async fn insert_official_submission(official_id: &str) -> Result<Option<String>> {
    let submission = api::get_submission_api(official_id).await?;
    eprintln!("Updating submission: {:#?}", &submission);
    let (submission_score, submission_error) = match submission.submission.score {
        api::SubmissionStatus::Processing => {
            let problem_id: u32 = submission.submission.problem_id;
            let input = api::get_problem(problem_id).await?.into();
            match parse_output(&submission.contents) {
                Ok(output) => {
                    let score = compute_score_fast(&input, &output).0;
                    eprintln!("Computed score: {}", score);
                    (Some(score), None)
                }
                Err(error) => (None, Some(format!("Local: {:?}", error))),
            }
        }
        api::SubmissionStatus::Success(score) => (Some(score as i64), None),
        api::SubmissionStatus::Failure(error) => (None, Some(error)),
    };
    sql::exec(
        "INSERT INTO submissions(
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
        ) ON DUPLICATE KEY UPDATE
            problem_id = :problem_id,
            submission_score = :submission_score,
            submission_error = :submission_error,
            submission_contents = :submission_contents,
            submission_created = :submission_created
        ",
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
            SELECT submission_id, submission_score, submission_error, problem_id
            FROM submissions
            WHERE official_id = :official_id",
            params! {
                "official_id" => &submission._id,
            },
        )?;
        let (mut submission_score, submission_error) = match submission.score {
            api::SubmissionStatus::Processing => (None, None),
            api::SubmissionStatus::Success(score) => (Some(score), None),
            api::SubmissionStatus::Failure(error) => (None, Some(error)),
        };
        let should_update = match result {
            Some(row) => {
                let db_submission_score: Option<u64> = row.get_option("submission_score")?;
                let db_submission_error: Option<String> = row.get_option("submission_error")?;
                if submission_score.is_none() && !db_submission_score.is_none() {
                    submission_score = db_submission_score
                }
                let is_processing = submission_score.is_none()
                    && submission_error.is_none()
                    && db_submission_score.is_none()
                    && db_submission_error.is_none();
                is_processing
                    || submission_score != db_submission_score
                    || submission_error != db_submission_error
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

pub async fn update_pending_official_submissions() -> Result<Vec<String>> {
    let mut ids = Vec::new();
    for row in sql::select(
        "
SELECT
    official_id
FROM
    submissions
WHERE
    submission_score IS NULL AND
    submission_error IS NULL AND
    official_id IS NOT NULL
ORDER BY
    RAND()
LIMIT 5",
        mysql::Params::Empty,
    )? {
        let official_id: String = row.get("official_id")?;
        eprintln!("Checking submission: {}", official_id);
        if let Some(id) = insert_official_submission(&official_id).await? {
            ids.push(id);
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
        let svg = vis::vis(&problem.into(), &(Vec::new(), Vec::new()), 1, !0, None);
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

    let mut rng = rand::thread_rng();
    let mut page_numbers = (2..=30).collect::<Vec<u32>>();
    page_numbers.shuffle(&mut rng);
    page_numbers.truncate(2);
    for i in 0..2 {
        page_numbers.push(i)
    }

    for i in page_numbers {
        match update_official_submissions(i * 100, 100).await {
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
    }
    match update_pending_official_submissions().await {
        Ok(ids) => {
            if ids.len() == 0 {
                buf.push_str("No pending submissions to update\n");
            } else {
                buf.push_str(&format!("Updated pending submissions: {:?}\n", ids));
            }
        }
        Err(e) => {
            buf.push_str(&format!("Failed to update pending submissions: {}\n", e));
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
