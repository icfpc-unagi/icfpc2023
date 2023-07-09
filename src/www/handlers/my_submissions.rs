use crate::*;

use actix_web::{web, HttpResponse, Responder};
use anyhow::Result;
use mysql::params;
use serde::Deserialize;
use std::fmt::Write;

#[derive(Deserialize)]
pub struct Query {
    #[serde(default = "default_offset")]
    pub offset: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_offset() -> u32 {
    0
}

fn default_limit() -> u32 {
    100
}

// Implement the handler function to list submissions from MySQL database with offset and limit.
pub async fn handler(info: web::Query<Query>) -> impl Responder {
    let mut buf = String::new();
    write!(
        &mut buf,
        "<a href=\"/submissions\">[Load from the official API]</a>"
    )
    .unwrap();
    write!(
        &mut buf,
        "<h1>Submissions from local DB copy (excl. pending submissions)</h1>"
    )
    .unwrap();
    buf.push_str("<table>");
    match sql::select(
        r#"
SELECT
    submission_id,
    official_id,
    problem_id,
    submission_score,
    submission_error,
    DATE_FORMAT(submission_created, "%Y-%m-%d %T") AS submission_created
FROM
    submissions
ORDER BY
    submission_created
DESC
LIMIT :offset, :limit
"#,
        params! {
            "offset" => info.offset,
            "limit" => info.limit
        },
    ) {
        Ok(rows) => {
            for row in rows {
                match || -> Result<String> {
                    let submission_id: u32 = row.get("submission_id")?;
                    let official_id: Option<String> = row.get_option("official_id")?;
                    let problem_id: u32 = row.get("problem_id")?;
                    let submission_score: Option<u32> = row.get_option("submission_score")?;
                    let submission_error: Option<String> = row.get_option("submission_error")?;
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
                    Ok(format!(
                        "<tr><td><a href=\"/submission?submission_id={}\">{}</a></td><td>{}</td><td>{}</td><td>{}</td></tr>",
                        submission_id,
                        official_id.unwrap_or("N/A".into()),
                        submission_created,
                        problem_id,
                        score,
                    ))
                }() {
                    Ok(s) => {
                        buf.push_str(&s);
                    }
                    Err(e) => {
                        buf.push_str(&format!("<tr><td>{}</td></tr>", e));
                    }
                };
            }
        }
        Err(e) => {
            buf.push_str(&format!("<tr><td>{}</td></tr>", e));
        }
    }
    buf.push_str("</table>");
    buf.push_str(
        format!(
            "<a href=\"/my_submissions?offset={}&limit={}\">Next</a>",
            info.offset + info.limit,
            info.limit
        )
        .as_str(),
    );
    HttpResponse::Ok()
        .content_type("text/html")
        .body(www::handlers::template::render(&buf))
}
