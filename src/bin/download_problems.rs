use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use reqwest::Error;
use serde_json::Value;
use anyhow::Result;
use tokio;

use icfpc2023::api;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let number_of_problems = api::get_number_of_problems().await.unwrap();
    let output_dir = "problems/";

    for problem_id in 1..=number_of_problems {
        let problem = api::get_raw_problem(problem_id).await;
        match problem {
            // Save problem to a file.
            Ok(json_string) => {
                let data_to_save: Value = serde_json::from_str(&json_string).unwrap_or(Value::Null);
                let output_path = format!("{}problem-{}.json", output_dir, problem_id);
                if Path::new(&output_path).exists() {
                    println!("File for problem_id={} already exists. Skipping...", problem_id);
                } else {
                    let mut file = File::create(output_path).unwrap();
                    file.write_all(data_to_save.to_string().as_bytes()).unwrap();
                    println!("Successfully downloaded and wrote data for problem_id={}", problem_id);
                }
            }
            Err(error) => {
                println!("problem_id={}: {}", problem_id, error);
            }
        }
    }

    Ok(())
}
