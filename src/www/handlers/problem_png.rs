use crate::*;

use actix_web::{web, Responder};
use anyhow::anyhow;
use mysql::params;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Query {
    pub problem_id: u32,
}

pub async fn handler(info: web::Query<Query>) -> impl Responder {
    match sql::cell(
        "SELECT problem_png_data FROM problem_pngs WHERE problem_id = :problem_id",
        params! {
            "problem_id" => info.problem_id
        },
    ) {
        Ok(Some(data)) => www::handlers::template::to_png_response(&data),
        Ok(None) => www::handlers::template::to_error_response(&anyhow!(
            "No such problem ID: {}",
            info.problem_id
        )),
        Err(e) => www::handlers::template::to_error_response(&e),
    }
}
