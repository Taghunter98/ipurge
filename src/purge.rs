use chrono::{Local, Utc};
use cron::Schedule;
use std::thread;
use std::str::FromStr;

pub fn run() {
    let expression: &'static str = "0/5 * * * * *";
    let schedule: Schedule = Schedule::from_str(expression).expect("Failed to parse CRON expression");

    loop {
        let now: chrono::DateTime<Utc> = Utc::now();
        if let Some(next) = schedule.upcoming(Utc).take(1).next() {
            let until_next = next - now;
            thread::sleep(until_next.to_std().unwrap());
            println!(
                "Running every 5 seconds. Current time: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S")
            );
        }
    }
}