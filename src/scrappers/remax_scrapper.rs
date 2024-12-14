use serde_json::json;
use std::time::Duration;
use thirtyfour::{By, WebDriver, WebElement};

use crate::schemas::remax_listing_raw::RemaxListingRaw;
use crate::scrappers::driver::initialize_driver;
use crate::utils::file_utils::*;
use thirtyfour::error::WebDriverError;
use tokio::fs::File;
use tokio::time::timeout;

async fn get_url_ids(web_driver: &WebDriver, page: u32) -> Result<Vec<String>, WebDriverError> {
    let _ = web_driver.get(format!("https://www.remax.pt/pt/comprar/imoveis/habitacao/r/r/r/t?s=%7B%7D&p={}&o=-ContractDate",page)).await;
    tokio::time::sleep(Duration::from_millis(2500)).await;

    let mut ids: Vec<String> = Vec::new();

    // The site takes some time to load this information
    while ids.is_empty() {
        let parent_div = web_driver.find(By::ClassName("pl-0")).await?;

        // Find all divs within it and get IDs
        let child_divs: Vec<WebElement> = parent_div.find_all(By::Tag("a")).await?;

        for div in child_divs {
            ids.push(div.attr("href").await?.unwrap());
        }
    }

    println!("{:?}", ids);

    Ok(ids)
}

async fn get_listing(
    web_driver: &WebDriver,
    url_id: String,
) -> Result<RemaxListingRaw, WebDriverError> {
    let _ = web_driver
        .get(format!("https://www.remax.pt/{}", url_id))
        .await;

    tokio::time::sleep(Duration::from_millis(200)).await;

    let description = web_driver.find(By::Id("description")).await?.text().await?;
    let details_div = web_driver.find(By::Id("details")).await?;
    let details_vec = details_div.find_all(By::ClassName("flex")).await?;

    let price: String = web_driver
        .find(By::Tag("main"))
        .await?
        .find(By::Tag("h2"))
        .await?
        .find(By::Tag("b"))
        .await?
        .text()
        .await?;

    let mut details_split_by_string: Vec<String> = Vec::new();

    for div in details_vec {
        details_split_by_string.push(div.text().await?);
    }

    let remax_listing_raw = RemaxListingRaw {
        price,
        description,
        details_split_by_string,
        url_id,
    };

    Ok(remax_listing_raw)
}

pub async fn remax_scrape_mechanism() -> Result<(), WebDriverError> {
    let web_driver: WebDriver = initialize_driver().await?;
    let remax_ids_read: File = get_file_read("remax_ids.txt").await?;
    let mut remax_ids_write: File = get_file_write("remax_ids.txt").await?;
    let mut remax_write: File = get_file_write("remax.json").await?;
    let remax_ids: String = get_content_as_string(remax_ids_read).await?;

    for page in 1.. {
        println!("\nScrapper mechanism page {}", page);

        // If we cannot get the page loaded in 30 seconds we ignore it and move on
        let url_ids_vec: Result<Vec<String>, WebDriverError> =
            timeout(Duration::from_secs(30), get_url_ids(&web_driver, page))
                .await
                .unwrap_or_else(|_| {
                    println!("Scrapper mechanism url_ids timed out after 30 seconds!");
                    Ok(Vec::new())
                });

        match url_ids_vec {
            Ok(url_ids) => {
                for url_id in url_ids {
                    if !remax_ids.contains(&url_id) {
                        println!("Url id: {}", url_id);
                        let remax_listing: RemaxListingRaw =
                            get_listing(&web_driver, url_id).await?;

                        write_to_file(&mut remax_write, format!("{}\n", json!(remax_listing)))
                            .await?;

                        write_to_file(&mut remax_ids_write, format!("{}\n", remax_listing.url_id))
                            .await?;

                        tokio::time::sleep(Duration::from_millis(500)).await;
                    } else {
                        println!("Url id: {} is already scrapped", url_id);
                    }
                }
            }
            Err(e) => {
                println!("Error getting url_ids: {}", e);
                break;
            }
        }
    }

    Ok(())
}

pub async fn run() {
    match remax_scrape_mechanism().await {
        Ok(_) => {
            println!("Remax scrapper mechanism finished")
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
