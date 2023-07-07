use actix_files::Files;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::env;

fn render(contents: &str) -> String {
    let mut s = String::new();
    s.push_str("<html lang=\"ja\">");
    s.push_str("<header>");
    s.push_str("<meta charset=\"utf-8\">");
    s.push_str("<meta name=\"viewport\" content=\"width=device-width,initial-scale=1.0,user-scalable=yes\">");
    s.push_str("<link rel=\"stylesheet\" type=\"text/css\" href=\"/static/style.css\">");
    s.push_str("<script src=\"https://ajax.googleapis.com/ajax/libs/jquery/3.3.1/jquery.min.js\"></script>");
    s.push_str("<script src=\"/static/jquery-linedtextarea.js\"></script>");
    s.push_str("<link href=\"/static/jquery-linedtextarea.css\" rel=\"stylesheet\"/>");
    s.push_str("</header>");
    s.push_str("<body>");
    s.push_str("<nav>");
    s.push_str("<a href=\"/\"></a>");
    s.push_str("<ul>");
    s.push_str("<li><a href=\"/visualizer\">可視化</a></li>");
    s.push_str("</ul>");
    s.push_str("</nav>");
    s.push_str("<main>");
    s.push_str("<article>");
    s.push_str(contents);
    s.push_str("</article>");
    s.push_str("</main>");
    s.push_str("</body>");
    s.push_str("</html>");
    s
}

async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(render("Hello, world!"))
}

async fn visualizer() -> impl Responder {
    let contents = std::fs::read_to_string("/www/visualizer.html").unwrap();
    let contents = contents.split("<body>").nth(1).unwrap().split("</body>").nth(0).unwrap();
    HttpResponse::Ok().content_type("text/html").body(render(contents))    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| String::from("0.0.0.0"));
    let server_port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    let bind_address = format!("{}:{}", server_address, server_port);

    eprintln!("Starting server at: {}", bind_address);
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
            .route("/visualizer", web::get().to(visualizer))
            .service(Files::new("/", "/www"))
    })
    .bind(bind_address)?
    .run()
    .await
}
