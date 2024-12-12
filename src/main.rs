use crate::scrappers::{era_scrapper, imovirtual_scrapper, remax_scrapper, supercasas_scrapper};
use dotenv::from_filename;
use std::env;

mod llms {
    pub mod llm_utils;
}

mod schemas {
    pub mod era_listing_raw;
    pub mod imovirtual_listing_raw;
    pub mod llm;
    pub mod remax_listing_raw;
    pub mod supercasas_listing_raw;
}
mod scrappers {
    pub mod driver;
    pub mod era_scrapper;
    pub mod file_utils;
    pub mod imovirtual_scrapper;
    pub mod remax_scrapper;
    pub mod supercasas_scrapper;
}

#[tokio::main]
async fn main() {
    let env_file: String = env::var("ENV_FILE").unwrap_or(".env".to_string());
    // Securely import sensitive credentials and values from your .env file
    from_filename(&env_file).ok();

    let mode: &str = &*env::var("MODE").expect("env variable `MODE` should be set");

    match mode {
        "remax" => {
            remax_scrapper::run().await;
        }
        "era" => {
            era_scrapper::run().await;
        }
        "supercasas" => {
            supercasas_scrapper::run().await;
        }
        "imovirtual" => {
            imovirtual_scrapper::run().await;
        }
        _ => {
            println!("Invalid mode provided. Use `remax`, `era`, `supercasas` or `imovirtual`.");
        }
    }
}
