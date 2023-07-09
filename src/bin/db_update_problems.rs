use icfpc2023::www;

#[tokio::main]
async fn main() {
    www::handlers::cron::update_official_problems()
        .await
        .unwrap();
}
