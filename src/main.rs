//! ipurge
//!
//! Software to purge unsued image directories for Whondo, to be run as a cronjob.
//!
//! ## Examples and Usage
//!
//! Create a .env file with the endpoint and folder destination for purging.
//!
//! ```plaintext
//! URL='https://myendpoint'
//! DIR_PATH='/home/files'
//! ```
//!
//! Then build and run the apllication passing the cronjob time as the argument.
//!
//! ## Examples
//! Run the cronjob once a day at midnight
//! ```bash
//! cargo run '0 0 * * * *' 'https://website.com/endpoint' '/home/user/purgedirectoty'
//! ```
//!
//! ## License
//!
//! Copyright (C) Josh Bassett, www.whondo.com. All rights reserved.
//!

use ftail::Ftail;
use log::LevelFilter;
use std::env;
use std::path::Path;

pub mod purge;

pub struct Config {
    runtime: String,
    endpoint: String,
    path: String,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config {
        runtime: args[1].trim().to_string(),
        endpoint: args[2].trim().to_string(),
        path: args[3].trim().to_string(),
    };

    let path: &Path = Path::new("./logs/purge.txt");

    Ftail::new()
        .single_file(path, true, LevelFilter::Trace)
        .init()
        .expect("unable to create logger");

    purge::run(config).await;
}
