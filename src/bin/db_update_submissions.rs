use icfpc2023::www;

#[tokio::main]
async fn main() {
    for i in 0..10 {
        www::handlers::cron::update_official_submissions(i * 100, 100)
            .await
            .unwrap();
    }
}
