use crate::*;

pub mod template;
pub mod visualize;
pub mod submission;
pub mod submissions;

use actix_web::{HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(www::handlers::template::render("Hello, world!"))
}

pub async fn visualizer() -> impl Responder {
    let contents = std::fs::read_to_string("/www/visualizer.html").unwrap();
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
