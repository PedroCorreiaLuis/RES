use crate::llms::llm_runner;
use crate::scrappers::{
    era_scrapper, idealista_scrapper, imovirtual_scrapper, remax_scrapper, supercasas_scrapper,
};
use dotenv::from_filename;
use std::env;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

mod llms {
    pub mod llm_runner;
    pub mod llm_utils;
}

mod schemas {
    pub mod era_listing_raw;
    pub mod idealista_listing_raw;
    pub mod imovirtual_listing_raw;
    pub mod llm;
    pub mod remax_listing_raw;
    pub mod supercasas_listing_raw;
}
mod scrappers {
    pub mod driver;
    pub mod era_scrapper;
    pub mod idealista_scrapper;
    pub mod imovirtual_scrapper;
    pub mod remax_scrapper;
    pub mod scrapper_utils;
    pub mod supercasas_scrapper;
}

mod utils {
    pub mod cache_utils;
    pub mod file_utils;
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
        "idealista" => {
            idealista_scrapper::run().await;
        }
        "llm" => {
            let llm_key: &str = &*env::var("OPEN_ROUTER_API_KEY")
                .expect("env variable `OPEN_ROUTER_API_KEY` should be set");
            let input: &str =
                &*env::var("INPUT_PATH").expect("env variable `INPUT_PATH` should be set");
            let output: &str =
                &*env::var("OUTPUT_PATH").expect("env variable `OUTPUT_PATH` should be set");

            let _ = llm_runner::run(llm_key, input, output).await;
        }
        _ => {
            println!("Invalid mode provided. Use `remax`, `era`, `supercasas`, idealista, `imovirtual` or llm.");
        }
    }
}
