use chrono::{Local, Utc};
use cron::Schedule;
use reqwest;
use serde_json::{self, Value};
use std::env::var;
use std::fs;
use std::str::FromStr;
use std::thread;

pub async fn run(runtime: &'static str) {;
    let schedule: Schedule =
        Schedule::from_str(runtime).expect("Failed to parse CRON expression");

    loop {
        let now: chrono::DateTime<Utc> = Utc::now();
        if let Some(next) = schedule.upcoming(Utc).take(1).next() {
            let until_next = next - now;
            thread::sleep(until_next.to_std().unwrap());
            log::info!(
                "Startomg purge: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S")
            );

            purge_heritics().await;

            log::info!(
                "Completed purge: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S")
            );
        }
    }
}

async fn purge_heritics() {
    let path: String = var("DIR_PATH").expect("unable to get value");

    println!("{:?}", path.as_str());

    let purge_list: Vec<String> = validate_directories(&path.as_str()).await;

    for heritic in purge_list {
        let heritic_path: String = format!("{}/{}", path.as_str(), heritic);

        match fs::remove_dir(heritic_path) {
            Ok(_) => log::info!("REM: {}", heritic),
            Err(_) => log::error!("Failed to remove {}", heritic),
        }
    }
}

async fn validate_directories(path: &str) -> Vec<String> {
    let emails: Vec<serde_json::Value> = parse_result().await;
    let mut purge_list: Vec<String> = Vec::new();

    for entry in fs::read_dir(path).expect("directory read failed") {
        let name: String = match entry.expect("entry name failed").file_name().into_string() {
            Ok(s) => s,
            Err(_) => continue,
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
    let url: String = std::env::var("URL").expect("unable to get value");

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let text = response.text().await?;
        Ok(text)
    } else {
        Err(format!("HTTP response: {}", response.status()).into())
    }
}
