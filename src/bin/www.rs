use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use icfpc2023::api;
use icfpc2023::www;
use serde::Deserialize;
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
    s.push_str("<li><a href=\"/submissions\">提出一覧</a></li>");
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
    HttpResponse::Ok()
        .content_type("text/html")
        .body(render("Hello, world!"))
}

async fn visualizer() -> impl Responder {
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
        .body(render(contents))
}

mod submissions {
    use super::*;

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
                for (i, submission) in submissions.iter().enumerate() {
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
                    .body(render(&format!("{}", e)));
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
            .body(render(&s))
    }
}

mod submission {
    use super::*;

    #[derive(Deserialize)]
    pub struct Query {
        pub submission_id: String,
    }

    pub async fn handler(info: web::Query<Query>) -> impl Responder {
        let mut s = String::new();
        match api::get_submission(&info.submission_id).await {
            Ok(submission) => {
                s.push_str(&format!(
                    "<h1>Submission ID: {}</h1>",
                    submission.submission._id
                ));
                s.push_str(&format!(
                    "<ul><li>Problem ID: {}</li>",
                    submission.submission.problem_id
                ));
                s.push_str(&format!(
                    "<li>Submitted at: {}</li>",
                    submission.submission.submitted_at
                ));
                let mut score_str = String::new();
                match &submission.submission.score {
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
                s.push_str(&format!("<li>Score: {}</li>", score_str));
                s.push_str(&format!(
                    "<pre style=\"white-space: pre-wrap;\"><code>{}</code></pre>",
                    submission.contents
                ));
            }
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .content_type("text/html")
                    .body(render(&format!("{}", e)));
            }
        }
        HttpResponse::Ok()
            .content_type("text/html")
            .body(render(&s))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| String::from("0.0.0.0"));
    let server_port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    let bind_address = format!("{}:{}", server_address, server_port);

    eprintln!("Starting server at: {}", bind_address);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/visualizer", web::get().to(visualizer))
            .route("/submissions", web::get().to(submissions::handler))
            .route("/submission", web::get().to(submission::handler))
            .route(
                "/visualize",
                web::get().to(www::handlers::visualize::handler),
            )
            .service(Files::new("/", "/www"))
    })
    .bind(bind_address)?
    .run()
    .await
}
