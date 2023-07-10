use crate::www::utils::maybe_enrich_datetime_str;
use crate::*;

use actix_web::{web, HttpResponse, Responder};
use anyhow::Result;
use mysql::params;
use num_format::{Locale, ToFormattedString};
use serde::Deserialize;
use std::iter::*;
use std::{collections::HashMap, fmt::Write};

#[derive(Deserialize)]
pub struct Query {
    #[serde(default = "default_offset")]
    pub offset: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub tag: String,
}

fn default_offset() -> u32 {
    0
}

fn default_limit() -> u32 {
    100
}

// Implement the handler function to list submissions from MySQL database with offset and limit.
pub async fn handler(info: web::Query<Query>) -> impl Responder {
    let response = match handle(&info).await {
        Ok(content) => HttpResponse::Ok()
            .content_type("text/html")
            .body(www::handlers::template::render(&content)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Internal server error: {}", e)),
    };

    response
}

async fn handle(info: &web::Query<Query>) -> Result<String, Box<dyn std::error::Error>> {
    let mut buf = String::new();
    write!(
        &mut buf,
        "<a href=\"/submissions\">[Load from the official API]</a>"
    )?;

    write!(
        &mut buf,
        "<h1>Submissions from local DB copy (excl. pending submissions)</h1>"
    )?;

    buf.push_str("<table>");

    let rows = if info.tag.is_empty() {
        sql::select(
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
        )?
    } else {
        sql::select(
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
NATURAL JOIN
    submission_tags
WHERE
    submission_tag = :tag
ORDER BY
    submission_created
DESC
LIMIT :offset, :limit
"#,
            params! {
                "tag" => &info.tag,
                "offset" => info.offset,
                "limit" => info.limit
            },
        )?
    };

    let submission_ids = rows
        .iter()
        .map(|row| {
            let submission_id: u32 = row.get("submission_id")?;
            Ok(submission_id.to_string())
        })
        .collect::<Result<Vec<_>>>()?;
    let tag_rows = sql::select(
        &format!(
            r#"
SELECT
    submission_id,
    submission_tag
FROM   
    submission_tags
WHERE
    submission_id IN ({})
"#,
            submission_ids.join(",")
        ),
        mysql::Params::Empty,
    )?;
    let mut tag_map = HashMap::<_, Vec<_>>::new();
    for row in tag_rows {
        let submission_id: u32 = row.get("submission_id")?;
        let submission_tag: String = row.get("submission_tag")?;
        tag_map
            .entry(submission_id)
            .or_default()
            .push(submission_tag);
    }

    for row in rows {
        match || -> Result<String> {
            let submission_id: u32 = row.get("submission_id")?;
            let official_id: Option<String> = row.get_option("official_id")?;
            let problem_id: u32 = row.get("problem_id")?;
            let submission_score: Option<i64> = row.get_option("submission_score")?;
            let submission_error: Option<String> = row.get_option("submission_error")?;
            let submission_created: String = row.get("submission_created")?;
            let score = match submission_score {
                Some(score) => {
                    format!("{}", score.to_formatted_string(&Locale::en))
                }
                None => match submission_error {
                    Some(error) => {
                        format!("{}", error)
                    }
                    None => "Processing".to_string(),
                },
            };
            let mut tag_str = String::new();
            if let Some(tags) = tag_map.get(&submission_id) {
                for tag in tags {
                    write!(
                        &mut tag_str,
                        "<a href=\"/my_submissions?tag={}\" class=\"tag\">{}</a>",
                        percent_encoding::utf8_percent_encode(
                            tag,
                            percent_encoding::NON_ALPHANUMERIC
                        ),
                        tag
                    )?;
                }
            }
            Ok(format!(
                "<tr><td><a href=\"/submission?submission_id={}\">{}</a></td><td>{}</td><td class=\"align-r\">{}</td><td class=\"align-r\">{}</td><td>{}</td></tr>",
                submission_id,
                official_id.unwrap_or(format!("{} (local-only)", submission_id)),
                maybe_enrich_datetime_str(submission_created),
                problem_id,
                score,
                tag_str,
            ))
        }() {
            Ok(s) => {
                buf.push_str(&s);
            }
            Err(e) => {
                write!(&mut buf, "<tr><td colspan=\"5\">{}</td></tr>", e)?;
            }
        }
    }

    buf.push_str("</table>");

    write!(
        &mut buf,
        "<a href=\"/my_submissions?offset={}&limit={}\">Next</a>",
        info.offset + info.limit,
        info.limit
    )?;

    Ok(buf)
}
