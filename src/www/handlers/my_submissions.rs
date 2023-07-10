use crate::www::utils::maybe_enrich_datetime_str;
use crate::*;

use actix_web::{web, HttpResponse, Responder};
use anyhow::Result;
use mysql::params;
use num_format::{Locale, ToFormattedString};
use serde::Deserialize;
use std::iter::*;
use std::{collections::HashMap, fmt::Write};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Query {
    #[serde(default = "default_offset")]
    pub offset: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub tag: String,
    #[serde(default = "default_problem_id")]
    pub problem_id: u32,
    #[serde(default)]
    pub order_by: String,
}

fn default_offset() -> u32 {
    0
}

fn default_limit() -> u32 {
    100
}

fn default_problem_id() -> u32 {
    0
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

pub fn build_url(info: &Query) -> String {
    format!(
        "/my_submissions?offset={offset}&limit={limit}&tag={tag}&problem_id={problem_id}&order_by={order_by}",
        offset = info.offset,
        limit = info.limit,
        tag = urlencode(&info.tag),
        problem_id = info.problem_id,
        order_by = urlencode(&info.order_by),
    )
}

async fn handle(info: &web::Query<Query>) -> Result<String, Box<dyn std::error::Error>> {
    let mut buf = String::new();
    write!(
        &mut buf,
        "<a href=\"/submissions\">[Load from the official API]</a>"
    )?;

    if info.problem_id == 0 {
        write!(&mut buf, "<h1>提出一覧 (DB)</h1>")?;
        let recent_tags = sql::select(
            r#"
SELECT submission_tag
FROM submission_tags
WHERE LEFT(submission_tag, 4) != 'CMD='
GROUP BY submission_tag
ORDER BY MAX(submission_tag_created) DESC
LIMIT 10"#,
            params::Params::Empty,
        )?
        .into_iter()
        .map(|row| row.get::<String>("submission_tag"))
        .collect::<Result<Vec<_>>>()?;
        for tag in &recent_tags {
            write!(
                &mut buf,
                " <a href=\"/my_submissions?tag={}\" class=\"tag\">{}</a>",
                urlencode(tag),
                tag,
            )?;
        }
    } else {
        write!(
            &mut buf,
            "<h1>問題番号: {problem_id}</h1>",
            problem_id = info.problem_id
        )?;
    }

    buf.push_str("<table>");

    let order_by = match info.order_by.as_str() {
        "submission_score" => "submission_score",
        _ => "submission_created",
    };

    let order_by_options = &[
        ("submission_created", "日付順"),
        ("submission_score", "スコア順"),
    ];
    // Enable to select order_by.
    buf.push_str("<div>ソート順:");
    for (column, name) in order_by_options {
        if order_by == column.deref() {
            write!(&mut buf, " [{name}]", name = name)?;
        } else {
            write!(
                &mut buf,
                " [<a href=\"{url}\">{name}</a>]",
                url = build_url(&Query {
                    order_by: column.to_string(),
                    ..info.deref().clone()
                }),
                name = name
            )?;
        }
    }
    buf.push_str("</div>");

    let rows = {
        let mut where_clause = Vec::new();
        if !info.tag.is_empty() {
            where_clause.push("submission_tag = :tag");
        }
        if info.problem_id != 0 {
            where_clause.push("problem_id = :problem_id");
        }
        let where_clause = if where_clause.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", where_clause.join(" AND "))
        };
        let where_clause = if info.tag.is_empty() {
            where_clause
        } else {
            format!(
                r#"NATURAL JOIN submission_tags {where_clause}"#,
                where_clause = where_clause
            )
        };
        sql::select(
            &format!(
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
{where_clause}
ORDER BY
    {order_by}
DESC
LIMIT :offset, :limit
"#,
                where_clause = where_clause,
                order_by = order_by
            ),
            params! {
                "tag" => &info.tag,
                "problem_id" => info.problem_id,
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
    let tag_rows = if submission_ids.is_empty() {
        Vec::new()
    } else {
        sql::select(
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
        )?
    };
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
                        "<a href=\"{url}\" class=\"tag\">{tag}</a>",
                        url = build_url(&Query {
                            tag: tag.clone(),
                            ..info.deref().clone()
                        }),
                        tag = tag
                    )?;
                }
            }
            Ok(format!(
                r#"
<tr>
<td><a href="/submission?submission_id={submission_id}">{official_id}</a></td>
<td>{submission_created}</td>
<td class="align-r"><a href="{problem_url}">{problem_id}</a></td>
<td class="align-r">{score}</td>
<td>{tags}</td>
</tr>"#,
                submission_id = submission_id,
                official_id = official_id.unwrap_or(format!("{} (local-only)", submission_id)),
                submission_created = maybe_enrich_datetime_str(submission_created),
                problem_url = build_url(&Query {
                    problem_id: problem_id,
                    ..info.deref().clone()
                }),
                problem_id = problem_id,
                score = score,
                tags = tag_str,
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
        "<a href=\"{url}\">Next</a>",
        url = build_url(&Query {
            offset: info.offset + info.limit,
            limit: info.limit,
            ..info.deref().clone()
        }),
    )?;

    Ok(buf)
}
