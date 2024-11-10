use crate::schemas::llm::ToLLMRequestBody;
use crate::scrappers::{era_scrapper, remax_scrapper};
use dotenv::dotenv;
use std::env;

mod llms {
    pub mod llm_utils;
}

mod schemas {
    pub mod era_listing_raw;
    pub mod llm;
    pub mod remax_listing_raw;
}
mod scrappers {
    pub mod driver;
    pub mod era_scrapper;
    pub mod file_utils;
    pub mod remax_scrapper;
}

#[tokio::main]
async fn main() {
    // Securely import sensitive credentials and values from your .env file
    dotenv().ok();

    let mode: &str = &*env::var("MODE").expect("env variable `MODE` should be set");

    match mode {
        "remax" => {
            remax_scrapper::run().await;
        }
        "era" => {
            era_scrapper::run().await;
        }
        _ => {
            println!("Invalid mode provided. Use `remax` or `era`");
        }
    }
}
