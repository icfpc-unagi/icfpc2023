use anyhow::Result;
use once_cell::sync::Lazy;
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use serde::Serialize;

const API_BASE: &str = "https://api.icfpcontest.com";

static TOKEN: Lazy<String> =
    Lazy::new(|| std::env::var("ICFPC2023_API_TOKEN").expect("ICFPC2023_API_TOKEN must be set"));
static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| reqwest::Client::new());

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum SubmissionStatus {
    Processing(),
    Success(usize),
    Failures(String),
}
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Submission {
    _id: String,
    problem_id: usize,
    submitted_at: String,
    score: SubmissionStatus,
}

pub async fn submissions() -> Result<Vec<Submission>> {
    let res = CLIENT
        .get(format!(
            "{}/submissions?offset={}&limit={}",
            API_BASE, 0, 10000
        ))
        .header(AUTHORIZATION, format!("Bearer {}", *TOKEN))
        .send()
        .await?;
    eprintln!("Status: {}", res.status());
    if !res.status().is_success() {
        panic!("Request failed: {}", res.status());
    }
    res.headers().iter().for_each(|(key, value)| {
        eprintln!("\t{}: {:?}", key, value);
    });
    let payload = res.bytes().await?;
    eprintln!("Payload: {}", String::from_utf8_lossy(&payload));
    #[derive(Deserialize)]
    enum Submissions {
        Success(Vec<Submission>),
        Failure(String),
    }
    let submissions: Submissions = serde_json::de::from_slice(&payload).unwrap();
    match submissions {
        Submissions::Success(submissions) => Ok(submissions),
        Submissions::Failure(error) => panic!("Unexpected response: {}", error),
    }
}
