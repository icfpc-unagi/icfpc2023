use crate::*;

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Query {
    pub submission_id: String,
}

pub async fn handler(info: web::Query<Query>) -> impl Responder {
    let mut buf = String::new();
    match api::get_submission(&info.submission_id).await {
        Ok(submission) => {
            buf.push_str(&format!(
                "<h1>Submission ID: {}</h1>",
                submission.submission._id
            ));
            buf.push_str(&format!(
                "<ul><li>Problem ID: {}</li>",
                submission.submission.problem_id
            ));
            buf.push_str(&format!(
                "<li>Submitted at: {}</li>",
                submission.submission.submitted_at
            ));
            buf.push_str(&format!("<li>Score: {}</li>", submission.submission.score));
            buf.push_str(&format!(
                "<pre style=\"white-space: pre-wrap;\"><code>{}</code></pre>",
                submission.contents
            ));
            let s = &submission.submission;
            // TODO(imos): Fill in the input.
            buf.push_str(&www::handlers::template::render_visualize(
                s.problem_id.try_into().unwrap_or(0),
                "",
                &submission.contents,
            ))
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
