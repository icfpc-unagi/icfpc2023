use crate::api;

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Query {
    pub submission_id: String,
}

pub async fn submission(info: web::Query<Query>) -> impl Responder {
    let submission = api::get_submission(&info.submission_id).await.unwrap();
    let json = serde_json::to_string(&submission).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(json)
}
