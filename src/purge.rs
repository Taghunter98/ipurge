//! This module provides an api to run a conjob to delete unused user directories.
//!
//! ## License
//!
//! Copyright (C) Josh Bassett, www.whondo.com. All rights reserved.
//!
//! Apache 2.0
//!

use chrono::{Local, Utc};
use cron::Schedule;
use reqwest;
use serde_json::{self, Value};
use std::fs;
use std::str::FromStr;
use std::thread;

use crate::Config;

/// Runs a cronjob that works through a list of directories and compares with email records from the database.
///
/// Function creates a new schedule and starts a continous loop running the cronjob as per user input.
/// 
/// The job runs the function `purge_heritics` that removes all unused directories and logs the purged emails.
///
/// # Panics
///
/// - If the `config.runtime` conversion fails.
/// - If the `Schedule` fails.
///
/// # Examples
/// ```rust
/// let config = Config {
///     runtime: "0 0 * * * *",
///     endpoint: "https://website.com/endpoint",
///     path: "/home/user/purgedirectory"
/// }
/// 
/// run(config)
/// ```
///
pub async fn run(config: Config) {
    let schedule: Schedule =
        Schedule::from_str(config.runtime.as_str()).expect("Failed to parse CRON expression");

    loop {
        let now: chrono::DateTime<Utc> = Utc::now();

        if let Some(next) = schedule.upcoming(Utc).take(1).next() {
            let until_next = next - now;
            thread::sleep(until_next.to_std().unwrap());
            log::info!(
                "Startomg purge: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S")
            );

            purge_heritics(&config).await;
        }
    }
}


/// Purges unused directories 'heritics' from the given path.
/// 
/// # Panics
///
/// - If the `Future` is not cxecuted successfully.
/// - If the `config.path` conversion fails.
/// - If the directory is not removable.
///
async fn purge_heritics(config: &Config) {
    let purge_list: Vec<String> = validate_directories(&config).await;

    for heritic in purge_list {
        let heritic_path: String = format!("{}/{}", config.path.as_str(), heritic);

        match fs::remove_dir(heritic_path) {
            Ok(_) => log::info!("REM: {}", heritic),
            Err(_) => log::error!("Failed to remove {}", heritic),
        }
    }
}

/// Validates directories and returns a purge list.
/// 
/// Function reviews all subdirectories of the `config.path` and compares with the email list.
/// 
/// The unused directories are added to a purge list to be remvoed.
/// 
/// # Panics
///
/// - If the `Future` is not executed succssfully.
/// - If the directory read fails.
///
async fn validate_directories(config: &Config) -> Vec<String> {
    let emails: Vec<Value> = parse_result(&config).await;
    let mut purge_list: Vec<String> = Vec::new();

    for entry in fs::read_dir(&config.path).expect("directory read failed") {
        let name: String = match entry.expect("entry name failed").file_name().into_string() {
            Ok(s) => s,
            Err(_) => continue,
        };

        if !emails.iter().any(|e: &Value| e[0].as_str() == Some(&name)) {
            purge_list.push(name);
        }
    }

    purge_list
}

/// Parses request into a `Vector<Value>` of emails.
/// 
/// # Panics
/// 
/// - If the `Future` is not cxecuted successfully.
/// - If the `Value` is not parsable.
/// - If the `Value` is not converted to a `Vec<Value>`.
///
async fn parse_result(config: &Config) -> Vec<Value> {
    let result: String = get_emails(&config).await.expect("unable to get result");
    let json: Value = serde_json::from_str(result.as_str()).expect("unable to parse result");

    json["emails"].as_array().unwrap().to_vec()
}

/// Makes a GET request to weatherapi.com.
/// 
/// # Errors
///
/// - If the GET request fails.
/// - If the response text fails.
/// - If the HTTP `response.status()` is not 200 OK.
///
async fn get_emails(config: &Config) -> Result<String, Box<dyn std::error::Error>> {
    let response: reqwest::Response = reqwest::get(config.endpoint.as_str()).await?;

    if response.status().is_success() {
        let text: String = response.text().await?;
        Ok(text)
    } else {
        Err(format!("HTTP response: {}", response.status()).into())
    }
}
