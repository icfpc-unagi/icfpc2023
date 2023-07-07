use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use reqwest::Error;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut problem_id = 1;
    let output_dir = "problems/";

    loop {
        let url = format!("https://api.icfpcontest.com/problem?problem_id={}", problem_id);
        let response = reqwest::get(&url).await?;
        if response.status().is_success() {
            let data: Value = response.json().await?;
            if let Some(success_data) = data.get("Success") {
                let json_string = success_data.as_str().unwrap_or("");
                let data_to_save: Value = serde_json::from_str(json_string).unwrap_or(Value::Null);
                let output_path = format!("{}problem-{}.json", output_dir, problem_id);
                if Path::new(&output_path).exists() {
                    println!("File for problem_id={} already exists. Skipping...", problem_id);
                } else {
                    let mut file = File::create(output_path).unwrap();
                    file.write_all(data_to_save.to_string().as_bytes()).unwrap();
                    println!("Successfully downloaded and wrote data for problem_id={}", problem_id);
                }
            } else {
                println!("'Success' key not found in response for problem_id={}. Skipping...", problem_id);
                break;
            }
        } else {
            println!("Download finished or failed. Check the last problem_id: {}", problem_id);
            break;
        }
        problem_id += 1;
    }
    Ok(())
}
