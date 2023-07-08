use crate::*;

use actix_files::Files;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::env;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Query {
    pub submission_id: String,
}

pub async fn handler(info: web::Query<Query>) -> impl Responder {
    let mut buf = String::new();
    match api::get_submission(&info.submission_id).await {
        Ok(submission) => {
            buf.push_str(&format!("<h1>Submission ID: {}</h1>", submission.submission._id));
            buf.push_str(&format!("<ul><li>Problem ID: {}</li>", submission.submission.problem_id));
            buf.push_str(&format!("<li>Submitted at: {}</li>", submission.submission.submitted_at));
            let mut score_str = String::new();
            match &submission.submission.score {
                api::SubmissionStatus::Processing => {
                    score_str.push_str("Pending");
                },
                api::SubmissionStatus::Success(score) => {
                    score_str.push_str(&format!("{}", score));
                },
                api::SubmissionStatus::Failure(e) => {
                    score_str.push_str(&format!("{}", e));
                },
            }
            buf.push_str(&format!("<li>Score: {}</li>", score_str));
            buf.push_str(&format!(
                "<pre style=\"white-space: pre-wrap;\"><code>{}</code></pre>",
                submission.contents));
            let s = &submission.submission;
            // TODO(imos): Fill in the input.
            buf.push_str(&www::handlers::template::render_visualize(
                s.problem_id.try_into().unwrap_or(0), "",
                &submission.contents))
        },
        Err(e) => {
            return HttpResponse::InternalServerError().content_type("text/html").body(www::handlers::template::render(&format!("{}", e)));
        }
    }
    HttpResponse::Ok().content_type("text/html").body(www::handlers::template::render(&buf))
}
