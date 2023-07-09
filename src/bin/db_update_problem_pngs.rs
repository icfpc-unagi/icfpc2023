use icfpc2023::www;

#[tokio::main]
async fn main() {
    www::handlers::cron::update_problem_png::update_all()
        .await
        .unwrap();
}
