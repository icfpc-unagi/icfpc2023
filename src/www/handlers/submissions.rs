use crate::*;

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

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
    let mut s = String::new();
    s.push_str("<table>");
    match api::get_submissions(info.offset, info.limit).await {
        Ok(submissions) => {
            for submission in submissions {
                let mut score_str = String::new();
                match &submission.score {
                    api::SubmissionStatus::Processing => {
                        score_str.push_str("Pending");
                    }
                    api::SubmissionStatus::Success(score) => {
                        score_str.push_str(&format!("{}", score));
                    }
                    api::SubmissionStatus::Failure(e) => {
                        score_str.push_str(&format!("{}", e));
                    }
                }
                s.push_str(&format!(
                    "<tr><td><a href=\"/submission?submission_id={}\">{}</a></td><td>{}</td><td>{}</td></tr>",
                    submission._id,
                    submission.submitted_at,
                    submission.problem_id,
                    score_str));
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(www::handlers::template::render(&format!("{}", e)));
        }
    }
    s.push_str("</table>");
    s.push_str(
        format!(
            "<a href=\"/submissions?offset={}&limit={}\">Next</a>",
            info.offset + info.limit,
            info.limit
        )
        .as_str(),
    );
    HttpResponse::Ok()
        .content_type("text/html")
        .body(www::handlers::template::render(&s))
}
