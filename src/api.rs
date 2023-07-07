use crate::*;

use anyhow::anyhow;
use anyhow::Result;
use once_cell::sync::Lazy;
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use serde::Serialize;

const API_BASE: &str = "https://api.icfpcontest.com";

static TOKEN: Lazy<String> = Lazy::new(|| secret::api_token().expect("UNAGI_PASSWORD must be set"));
static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| reqwest::Client::new());

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum SubmissionStatus {
    Processing,
    // Score must be an integer but it has floating point in json for some reason.
    Success(#[serde(deserialize_with = "parse_u64_via_f64")] u64),
    Failure(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Submission {
    _id: String,
    problem_id: usize,
    submitted_at: String,
    score: SubmissionStatus,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
enum Response<T> {
    Success(T),
    Failure(String),
}

/// Returns the number of problems in the contest.
/// Authentication is not required.
pub async fn get_number_of_problems() -> Result<u32> {
    let res = CLIENT.get(format!("{}/problems", API_BASE)).send().await?;
    eprintln!("Status: {}", res.status());
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
        .await?;
    eprintln!("Status: {}", res.status());
    let problem_response: Response<String> = res.json().await?;
    match problem_response {
        Response::Success(problem) => Ok(problem),
        Response::Failure(error) => Err(anyhow!(error)),
    }
}

/// Returns the problem with the given ID.
/// Authentication is not required.
pub async fn get_problem(problem_id: u32) -> Result<JsonConcert> {
    let problem = get_raw_problem(problem_id).await?;
    let problem: JsonConcert = serde_json::from_str(&problem)?;
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
        .await?;
    eprintln!("Status: {}", res.status());
    Ok(res.json().await?)
}

pub async fn get_userboard() -> Result<Vec<Option<u64>>> {
    let res = CLIENT
        .get(format!("{}/userboard", API_BASE))
        .header(AUTHORIZATION, format!("Bearer {}", *TOKEN))
        .send()
        .await?;
    eprintln!("Status: {}", res.status());
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
        .header(AUTHORIZATION, format!("Bearer {}", *TOKEN))
        .send()
        .await?;
    eprintln!("Status: {}", res.status());
    let submissions: Response<Vec<Submission>> = res.json().await?;
    match submissions {
        Response::Success(submissions) => Ok(submissions),
        Response::Failure(error) => Err(anyhow!(error)),
    }
}

/// Submits a solution and returns the submission ID.
pub async fn submit(problem_id: u32, placements: &Output) -> Result<u32> {
    #[derive(Serialize)]
    struct Solution {
        placements: Vec<XY>,
    }
    let contents = serde_json::to_string(&Solution {
        placements: placements.iter().map(|p| p.into()).collect(),
    })?;

    submit_raw(problem_id, &contents).await
}

/// Submits a solution and returns the submission ID.
pub async fn submit_raw(problem_id: u32, contents: &str) -> Result<u32> {
    #[derive(Serialize)]
    struct SubmissionRequest<'a> {
        problem_id: u32,
        contents: &'a str,
    }
    let request = SubmissionRequest {
        problem_id,
        contents: contents,
    };
    let res = CLIENT
        .post(format!("{}/submission", API_BASE))
        .header(AUTHORIZATION, format!("Bearer {}", *TOKEN))
        .body(serde_json::to_vec(&request)?)
        .send()
        .await?;
    eprintln!("Status: {}", res.status());
    let submission_id: u32 = res.text().await?.parse()?;
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
