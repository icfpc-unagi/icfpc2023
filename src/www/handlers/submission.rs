use crate::*;

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Query {
    pub submission_id: String,
    #[serde(default = "default_color_type")]
    pub color_type: i32,
}

fn default_color_type() -> i32 {
    1
}

// use actix_web::web;
// use actix_web::HttpResponse;
// use actix_web::Responder;
use std::fmt::Write;

pub async fn handler(info: web::Query<Query>) -> impl Responder {
    let mut buf = String::new();
    match api::get_submission(&info.submission_id).await {
        Ok(submission) => {
            write!(
                &mut buf,
                "<h1>Submission ID: {}</h1>",
                submission.submission._id
            );
            write!(
                &mut buf,
                "<ul><li>Problem ID: {}</li>",
                submission.submission.problem_id
            );
            write!(
                &mut buf,
                "<li>Submitted at: {}</li>",
                submission.submission.submitted_at
            );
            let score_str = match &submission.submission.score {
                api::SubmissionStatus::Processing => {
                    format!("Pending")
                }
                api::SubmissionStatus::Success(score) => {
                    format!("{}", score)
                }
                api::SubmissionStatus::Failure(e) => {
                    format!("{}", e)
                }
            };
            write!(&mut buf, "<li>Score: {}</li>", score_str);
            let problem_id = submission.submission.problem_id;
            // TODO: Cache problem data
            let input: Input = api::get_problem(problem_id).await.unwrap().into();
            let output = parse_output(&submission.contents);
            let svg = vis::vis(&input, &output, info.color_type);
            write!(&mut buf, "{}", svg.2);
            write!(
                &mut buf,
                "<pre style=\"white-space: pre-wrap;\"><code>{}</code></pre>",
                submission.contents
            );
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(www::handlers::template::render(&format!("{}", e)));
        }
    }
    HttpResponse::Ok()
        .content_type("text/html")
        .body(www::handlers::template::render(&buf))
}
