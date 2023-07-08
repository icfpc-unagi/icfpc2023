use crate::*;

use anyhow::anyhow;
use anyhow::Result;
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

/// Returns the problem with the given ID.
/// Authentication is not required.
pub async fn get_problem(problem_id: u32) -> Result<Problem> {
    let problem = get_raw_problem(problem_id).await?;
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

pub async fn get_submission(submission_id: &str) -> Result<SubmissionResponse> {
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

/// Submits a solution and returns the submission ID.
pub async fn submit(problem_id: u32, placements: &Output) -> Result<String> {
    let contents = serde_json::to_string(&Solution {
        placements: placements.iter().map(|p| p.into()).collect(),
    })?;

    submit_raw(problem_id, &contents).await
}

/// Submits a solution and returns the submission ID.
pub async fn submit_raw(problem_id: u32, contents: &str) -> Result<String> {
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
    let submission_id = res.text().await?;
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
