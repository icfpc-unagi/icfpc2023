use crate::*;

use actix_web::{web, HttpResponse, Responder};
use anyhow::anyhow;
use anyhow::Result;
use mysql::params;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Query {
    pub submission_id: u32,
    #[serde(default = "default_color_type")]
    pub color_type: i32,
}

fn default_color_type() -> i32 {
    1
}

async fn get_submission(info: web::Query<Query>) -> Result<String> {
    let row = sql::row(
        r#"
SELECT
    submission_id,
    official_id,
    problem_id,
    submission_score,
    submission_error,
    submission_contents,
    DATE_FORMAT(submission_created, "%Y-%m-%d %T") AS submission_created
FROM
    submissions
WHERE
    submission_id = :submission_id"#,
        params! {
            "submission_id" => info.submission_id
        },
    )?
    .ok_or(anyhow!("エラー: 該当の提出 ID が見つかりませんでした。"))?;

    let submission_id: u32 = row.get("submission_id")?;
    let official_id: Option<String> = row.get_option("official_id")?;
    let problem_id: u32 = row.get("problem_id")?;
    let submission_score: Option<u32> = row.get_option("submission_score")?;
    let submission_error: Option<String> = row.get_option("submission_error")?;
    let submission_contents: Option<String> = row.get_option("submission_contents")?;
    let submission_created: String = row.get("submission_created")?;
    let score = match submission_score {
        Some(score) => {
            format!("{}", score)
        }
        None => match submission_error {
            Some(error) => {
                format!("{}", error)
            }
            None => "Processing".to_string(),
        },
    };
    let contents = match submission_contents {
        Some(contents) => {
            format!("{}", contents)
        }
        None => "N/A".to_string(),
    };
    let mut buf = String::new();
    buf.push_str(&format!("<h1>Submission ID: {}</h1>", submission_id));
    buf.push_str(&format!("<ul><li>Problem ID: {}</li>", problem_id));
    buf.push_str(&format!("<li>Score: {}</li>", score));
    buf.push_str(&format!(
        "<li>Official ID: {}</li>",
        official_id.unwrap_or("N/A".into())
    ));
    buf.push_str(&format!("<li>Created at: {}</li></ul>", submission_created));
    let input: Input = api::get_problem(problem_id).await.unwrap().into();
    let output = parse_output(&contents)?;
    let svg = vis::vis(&input, &output, info.color_type, !0);
    buf.push_str(&svg.2);
    buf.push_str(&format!(
        "<pre style=\"white-space: pre-wrap;\"><code>{}</code></pre>",
        contents
    ));
    Ok(buf)
}

pub async fn handler(info: web::Query<Query>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(www::handlers::template::render(
            &match get_submission(info).await {
                Ok(buf) => buf,
                Err(e) => format!("<h1>エラー</h1><p>{}</p>", e),
            },
        ))
}
