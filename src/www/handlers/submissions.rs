use crate::*;

use actix_web::{web, HttpResponse, Responder};
use anyhow::Result;
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

pub async fn handler(info: web::Query<Query>) -> impl Responder {
    match handle(info).await {
        Ok(contents) => HttpResponse::Ok()
            .content_type("text/html")
            .body(www::handlers::template::render(&contents)),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(www::handlers::template::render(&format!("{}", e))),
    }
}

pub async fn handle(info: web::Query<Query>) -> Result<String> {
    let mut s = String::new();
    write!(&mut s, "<table>")?;
    let submissions = api::get_submissions(info.offset, info.limit).await?;
    for submission in submissions {
        write!(
            &mut s,
            "<tr><td><a href=\"/submission?submission_id={}\">{}</a></td><td>{}</td><td>{}</td><td>{}</td></tr>",
            submission._id,
            submission._id,
            submission.submitted_at,
            submission.problem_id,
            submission.score,
        )?;
    }
    write!(&mut s, "</table>")?;
    write!(
        &mut s,
        "<a href=\"/submissions?offset={}&limit={}\">Next</a>",
        info.offset + info.limit,
        info.limit
    )?;
    Ok(s)
}
