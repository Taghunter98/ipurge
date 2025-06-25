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
//! Then build and run the apllication, ipurge will then run
//!
//! ## License
//!
//! Copyright (C) Josh Bassett, www.whondo.com. All rights reserved.
//!

use dotenv::dotenv;
use ftail::Ftail;
use log::LevelFilter;
use std::path::Path;
pub mod purge;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let path: &Path = Path::new("./logs/purge.txt");

    Ftail::new()
        .single_file(path, true, LevelFilter::Trace)
        .init()
        .expect("unable to create logger");

    purge::run("0/5 * * * * *").await; // add this as args
}
