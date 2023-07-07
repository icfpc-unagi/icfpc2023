use actix_files::Files;
use actix_web::{web, App, HttpServer, Responder};
use std::env;

async fn index() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| String::from("0.0.0.0"));
    let server_port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    let bind_address = format!("{}:{}", server_address, server_port);

    eprintln!("Starting server at: {}", bind_address);
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
            .service(Files::new("/", "/www"))
    })
    .bind(bind_address)?
    .run()
    .await
}
