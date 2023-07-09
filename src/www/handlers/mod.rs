use crate::*;

pub mod api_proxy;
pub mod cron;
pub mod my_submission;
pub mod my_submissions;
pub mod my_userboard;
pub mod submission;
pub mod submissions;
pub mod template;
pub mod visualize;

use actix_web::{HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(www::handlers::template::render("Hello, world!"))
}

pub async fn visualizer() -> impl Responder {
    let contents = std::fs::read_to_string("vis/index.html").unwrap();
    let contents = contents
        .split("<body>")
        .nth(1)
        .unwrap()
        .split("</body>")
        .nth(0)
        .unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(www::handlers::template::render(contents))
}
