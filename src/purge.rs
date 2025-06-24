use chrono::{Local, Utc};
use cron::Schedule;
use reqwest;
use serde_json::{self, Value};
use std::fs;
use std::str::FromStr;
use std::thread;

pub async fn run() {
    let expression: &'static str = "0/5 * * * * *";
    let schedule: Schedule =
        Schedule::from_str(expression).expect("Failed to parse CRON expression");

    loop {
        let now: chrono::DateTime<Utc> = Utc::now();
        if let Some(next) = schedule.upcoming(Utc).take(1).next() {
            let until_next = next - now;
            thread::sleep(until_next.to_std().unwrap());
            println!(
                "Running cronjob. Current time: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S")
            );

            let path = std::env::var("PATH").expect("unable to get value");

            purge_heritics(path.as_str()).await;
        }
    }
}

async fn purge_heritics(path: &str) {
    let purge_list: Vec<String> = validate_directories(path).await;
    for heritic in purge_list {
        let heritic_path = format!("{path}/{heritic}");
        match fs::remove_dir(heritic_path) {
            Ok(r) => println!("Directory removed"),
            Err(_) => println!("Directory not removed")
        }
    }
}

async fn validate_directories(path: &str) -> Vec<String> {
    let emails: Vec<serde_json::Value> = parse_result().await;
    let mut purge_list: Vec<String> = Vec::new();

    for entry in fs::read_dir(path).expect("directory read failed") {
        let name = match entry.expect("entry name failed").file_name().into_string() {
            Ok(s) => s,
            Err(_) => continue
        };
        
        if !emails.iter().any(|e| e[0].as_str() == Some(&name)) {
            purge_list.push(name);
        }

    }

    purge_list
}

async fn parse_result() -> Vec<Value> {
    let result: String = get_emails().await.expect("unable to get result");
    let json: Value = serde_json::from_str(result.as_str()).expect("unable to parse result");

    json["emails"].as_array().unwrap().to_vec()
}

async fn get_emails() -> Result<String, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let url: String = std::env::var("URL").expect("unable to get value");

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let text = response.text().await?;
        Ok(text)
    } else {
        Err(format!("HTTP response: {}", response.status()).into())
    }
}
