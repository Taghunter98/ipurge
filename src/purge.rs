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
            validate_directories().await;
        }
    }
}

async fn validate_directories() {
    let emails: Vec<serde_json::Value> = parse_result().await;
    let mut purge_list: Vec<String> = Vec::new();

    for e in emails.iter() {
        let email_str: &str = e[0].as_str().expect("unable to convert to &str");
        let path: String = format!("/home/josh/Documents/ipurge/{email_str}");

        if !directory_exists(&path) {
            purge_list.push(path);
        }
    }

    println!("Directories to be purged");
    for p in purge_list.iter() {
        println!("{p}");
    }
}

fn directory_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
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
