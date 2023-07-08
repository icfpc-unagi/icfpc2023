use actix_files::Files;
use actix_web::{web, App, HttpServer};
use icfpc2023::www;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| String::from("0.0.0.0"));
    let server_port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    let bind_address = format!("{}:{}", server_address, server_port);

    eprintln!("Starting server at: {}", bind_address);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(www::handlers::index))
            .route("/visualizer", web::get().to(www::handlers::visualizer))
            .route("/submissions", web::get().to(www::handlers::submissions::handler))
            .route(
                "/submission",
                web::get().to(www::handlers::submission::handler),
            )
            .route(
                "/visualize",
                web::get().to(www::handlers::visualize::handler),
            )
            .route(
                "/my_userboard",
                web::get().to(www::handlers::my_userboard::handler),
            )
            .route(
                "/my_submissions",
                web::get().to(www::handlers::my_submissions::handler),
            )
            .route(
                "/cron",
                web::get().to(www::handlers::cron::handler),
            )
            .service(Files::new("/", "/www"))
    })
    .bind(bind_address)?
    .run()
    .await
}
