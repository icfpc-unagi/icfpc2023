use std::fmt;

use crate::*;

use anyhow::anyhow;
use anyhow::Result;
use mysql::params;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;

const API_BASE: &str = "https://api.icfpcontest.com";

static TOKEN: Lazy<String> = Lazy::new(|| secret::api_token().expect("UNAGI_PASSWORD must be set"));
static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| reqwest::Client::new());

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum SubmissionStatus {
    Processing,
    Failure(String),
    // Score must be an integer but it has floating point in json for some reason.
    Success(#[serde(deserialize_with = "parse_u64_via_f64")] u64),
}

impl fmt::Display for SubmissionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubmissionStatus::Processing => write!(f, "Pending"),
            SubmissionStatus::Success(score) => write!(f, "{}", score),
            SubmissionStatus::Failure(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Submission {
    pub _id: String,
    pub problem_id: u32,
    pub submitted_at: String,
    pub score: SubmissionStatus,
    // NOTE: This field is not documented in the API spec.
    pub user_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct SubmissionResponse {
    pub submission: Submission,
    pub contents: String,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
enum Response<T> {
    Success(T),
    Failure(String),
}

/// Returns the number of problems in the contest.
/// Authentication is not required.
pub async fn get_number_of_problems() -> Result<u32> {
    let res = CLIENT
        .get(format!("{}/problems", API_BASE))
        .send()
        .await?
        .error_for_status()?;
    #[derive(Deserialize)]
    struct ProblemsResponse {
        number_of_problems: u32,
    }
    let problem_response: ProblemsResponse = res.json().await?;
    Ok(problem_response.number_of_problems)
}

/// Returns the problem with the given ID.
/// Authentication is not required.
pub async fn get_raw_problem(problem_id: u32) -> Result<String> {
    let res = CLIENT
        .get(format!("{}/problem?problem_id={}", API_BASE, problem_id))
        .send()
        .await?
        .error_for_status()?;
    let problem_response: Response<String> = res.json().await?;
    match problem_response {
        Response::Success(problem) => Ok(problem),
        Response::Failure(error) => Err(anyhow!(error)),
    }
}

// Returns the problem but from CDN.
pub async fn get_raw_problem_cdn(problem_id: u32) -> Result<String> {
    let res = CLIENT
        .get(format!(
            "https://cdn.icfpcontest.com/problems/{}.json",
            problem_id
        ))
        .send()
        .await?
        .error_for_status()?;
    Ok(res.text().await?)
}

// Returns the problem but from DB.
pub async fn get_raw_problem_db(problem_id: u32) -> Result<String> {
    let mut chunks = String::new();
    let rows = sql::select(
        "
        SELECT problem_chunk_index, problem_chunk
        FROM problem_chunks
        WHERE problem_id = :problem_id
        ORDER BY problem_chunk_index",
        params! {
            "problem_id" => problem_id
        },
    )?;
    if rows.len() == 0 {
        return Err(anyhow::anyhow!("Problem {} not found", problem_id));
    }
    for (index, row) in rows.iter().enumerate() {
        let chunk_index: usize = row.get("problem_chunk_index")?;
        if index != chunk_index {
            return Err(anyhow!("Problem chunk index mismatch"));
        }
        let chunk: String = row.get::<String>("problem_chunk")?;
        chunks.push_str(&chunk);
    }
    Ok(chunks)
}

/// Returns the problem with the given ID.
/// Authentication is not required.
pub async fn get_problem(problem_id: u32) -> Result<Problem> {
    // Try DB first.
    let problem = match get_raw_problem_db(problem_id).await {
        Ok(problem) => problem,
        // Fallback to CDN.
        Err(_) => get_raw_problem_cdn(problem_id).await?,
    };
    let problem: Problem = serde_json::from_str(&problem)?;
    Ok(problem)
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Scoreboard {
    frozen: bool,
    scoreboard: Vec<ScoreboardEntry>,
    updated_at: String,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ScoreboardEntry {
    username: String,
    score: u64,
}

/// Returns the global scoreboard.
/// Authentication is not required.
pub async fn get_scoreboard() -> Result<Scoreboard> {
    let res = CLIENT
        .get(format!("{}/scoreboard", API_BASE))
        .send()
        .await?
        .error_for_status()?;
    Ok(res.json().await?)
}

/// Returns the user's highest scores for all problems or None if no scored submissions.
pub async fn get_userboard() -> Result<Vec<Option<u64>>> {
    let res = CLIENT
        .get(format!("{}/userboard", API_BASE))
        .bearer_auth(&*TOKEN)
        .send()
        .await?
        .error_for_status()?;
    #[derive(Deserialize)]

    struct Userboard {
        problems: Vec<Option<u64>>,
    }
    let userboard_response: Response<Userboard> = res.json().await?;
    match userboard_response {
        Response::Success(userboard) => Ok(userboard.problems),
        Response::Failure(error) => Err(anyhow!(error)),
    }
}

/// Returns the submissions with the given offset and limit.
pub async fn get_submissions(offset: u32, limit: u32) -> Result<Vec<Submission>> {
    let res = CLIENT
        .get(format!(
            "{}/submissions?offset={}&limit={}",
            API_BASE, offset, limit
        ))
        .bearer_auth(&*TOKEN)
        .send()
        .await?
        .error_for_status()?;
    let submissions: Response<Vec<Submission>> = res.json().await?;
    match submissions {
        Response::Success(submissions) => Ok(submissions),
        Response::Failure(error) => Err(anyhow!(error)),
    }
}

pub async fn get_submission_db(submission_id: &str) -> Result<SubmissionResponse> {
    let local_id = submission_id.parse().unwrap_or(0);
    let official_id = submission_id;
    let rows = sql::select(
        "
SELECT
    submission_id,
    official_id,
    problem_id,
    submission_score,
    submission_error,
    submission_contents,
    DATE_FORMAT(submission_created, \"%Y-%m-%d %T\") AS submission_created
FROM
    submissions
WHERE
    submission_id = :local_id OR official_id = :official_id",
        params! {
            "local_id" => local_id,
            "official_id" => official_id,
        },
    )?;
    let row = rows
        .first()
        .ok_or(anyhow!("エラー: 該当の提出 ID が見つかりませんでした。"))?;

    let submission_id: u32 = row.get("submission_id")?;
    let official_id: Option<String> = row.get_option("official_id")?;
    let problem_id: u32 = row.get("problem_id")?;
    let submission_score: Option<u64> = row.get_option("submission_score")?;
    let submission_error: Option<String> = row.get_option("submission_error")?;
    let submission_contents: Option<String> = row.get_option("submission_contents")?;
    let submission_created: String = row.get("submission_created")?;
    Ok(SubmissionResponse {
        submission: Submission {
            _id: official_id.unwrap_or(submission_id.to_string()),
            problem_id: problem_id,
            submitted_at: submission_created,
            score: match (submission_score, submission_error) {
                (Some(score), _) => SubmissionStatus::Success(score),
                (_, Some(error)) => SubmissionStatus::Failure(error),
                _ => SubmissionStatus::Processing,
            },
            user_id: None,
        },
        contents: submission_contents.unwrap_or_default(),
    })
}

pub async fn get_submission_api(submission_id: &str) -> Result<SubmissionResponse> {
    let res = CLIENT
        .get(format!(
            "{}/submission?submission_id={}",
            API_BASE, submission_id
        ))
        .bearer_auth(&*TOKEN)
        .send()
        .await?;
    let submission: Response<SubmissionResponse> = res.json().await?;
    match submission {
        Response::Success(submission) => Ok(submission),
        Response::Failure(error) => Err(anyhow!(error)),
    }
}

/// Returns the submission with the given ID of either local or offical format.
pub async fn get_submission(submission_id: &str) -> Result<SubmissionResponse> {
    // Try to get from DB.
    match get_submission_db(submission_id).await {
        Ok(submission) => return Ok(submission),
        Err(error) => eprintln!("Failed to get submission from DB: {}", error),
    }
    get_submission_api(submission_id).await
}

/// Registers tags for a submission.
/// It does not remove existing tags.
async fn tag_submission(local_submission_id: u64, tags: &[&str]) -> Result<()> {
    if tags.is_empty() {
        return Ok(());
    }
    let local_id = local_submission_id;
    let official_id = local_submission_id;
    sql::exec_batch(
        "
INSERT IGNORE INTO submission_tags
    (submission_id, submission_tag)
VALUES
    (:local_id, :submission_tag)
",
        tags.iter().map(|&tag| {
            params! {
                "local_id" => local_id,
                "official_id" => official_id,
                "submission_tag" => tag,
            }
        }),
    )?;
    Ok(())
}

/// Submits a solution and returns the submission ID.
pub async fn submit(
    problem_id: u32,
    output: &Output,
    tags: &[&str],
    local: bool,
) -> Result<String> {
    if local {
        let local_id = submit_local(problem_id, output).await?;
        tag_submission(local_id, tags).await?;
        return Ok(local_id.to_string());
    } else {
        let official_id = submit_api(problem_id, output).await?;
        let local_id = insert_placeholder_submission(problem_id, &official_id).await?;
        tag_submission(local_id, tags).await?;
        return Ok(official_id);
    }
}

/// Inserts a placeholder submission to local DB and returns the local submission ID.
pub async fn insert_placeholder_submission(problem_id: u32, official_id: &str) -> Result<u64> {
    let local_id = sql::insert(
        "
INSERT IGNORE INTO submissions
    (problem_id, official_id)
VALUES
    (:problem_id, :official_id)",
        params! {
            "problem_id" => problem_id,
            "official_id" => official_id,
        },
    )?;
    Ok(local_id)
}

/// Submits a solution to local DB and returns the local submission ID.
/// It fails if the submission is not valid.
pub async fn submit_local(problem_id: u32, output: &Output) -> Result<u64> {
    let input = get_problem(problem_id).await?.into();
    // TODO(sulume): Find the right scoring.
    let score = compute_score(&input, &output);
    let contents = serde_json::to_string::<Solution>(&output.into())?;
    let local_id = sql::insert(
        "
INSERT INTO submissions
    (problem_id, submission_score, submission_contents)
VALUES
    (:problem_id, :submission_score, :submission_contents)",
        params! {
            "problem_id" => problem_id,
            "submission_score" => score,
            "submission_contents" => contents,
        },
    )?;
    Ok(local_id)
}

/// Submits a solution and returns the submission ID.
pub async fn submit_api(problem_id: u32, output: &Output) -> Result<String> {
    let contents = serde_json::to_string::<Solution>(&output.into())?;
    let submission_id = submit_raw_api(problem_id, &contents).await?;
    Ok(submission_id)
}

/// Submits a solution and returns the submission ID.
pub async fn submit_raw_api(problem_id: u32, contents: &str) -> Result<String> {
    #[derive(Serialize)]
    struct SubmissionRequest<'a> {
        problem_id: u32,
        contents: &'a str,
    }
    let request = SubmissionRequest {
        problem_id,
        contents,
    };
    let res = CLIENT
        .post(format!("{}/submission", API_BASE))
        .bearer_auth(&*TOKEN)
        .json(&request)
        .send()
        .await?
        .error_for_status()?;
    let submission_id = res.text().await?.trim_matches('"').into();
    Ok(submission_id)
}

fn parse_u64_via_f64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let value: f64 = serde::de::Deserialize::deserialize(deserializer)?;
    Ok(value as u64)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_json_submissions() {
        let json = r#"
          {
            "Success": [
              {
                "_id": "xxxx",
                "problem_id": 1,
                "submitted_at": "2023-07-07T14:50:28.397437731Z",
                "score": "Processing"
              },
              {
                "_id": "yyyy",
                "problem_id": 2,
                "submitted_at": "2023-07-07T14:50:28.397437731Z",
                "score": {
                  "Success": 0
                }
              },
              {
                "_id": "zzzz",
                "problem_id": 3,
                "submitted_at": "2023-07-07T14:50:28.397437731Z",
                "score": {
                  "Failure": "error message"
                }
              }
            ]
          }
        "#;
        let parsed: super::Response<Vec<super::Submission>> = serde_json::from_str(json).unwrap();
        if let super::Response::Success(parsed) = parsed {
            assert_eq!(parsed.len(), 3);
        }
    }
}
