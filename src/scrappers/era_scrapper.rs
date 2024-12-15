use crate::schemas::era_listing_raw::EraListingRaw;
use crate::scrappers::driver::initialize_driver;
use crate::utils::file_utils::*;
use serde_json::json;
use std::time::Duration;
use thirtyfour::error::WebDriverError;
use thirtyfour::{By, WebDriver, WebElement};
use tokio::fs::File;
use tokio::time::timeout;

async fn get_url_ids(web_driver: &WebDriver, page: u32) -> Result<Vec<String>, WebDriverError> {
    let _ = web_driver
        .get(format!(
            "https://www.era.pt/comprar?ob=1&tp=1,2&page={}&ord=3",
            page
        ))
        .await;
    tokio::time::sleep(Duration::from_millis(2500)).await;

    let mut ids: Vec<String> = Vec::new();

    // The site takes some time to load this information
    while ids.is_empty() {
        // Find all divs within it and get IDs
        let child_divs: Vec<WebElement> = web_driver.find_all(By::ClassName("card")).await?;

        for div in child_divs {
            ids.push(div.find(By::Tag("a")).await?.attr("href").await?.unwrap());
        }
    }

    println!("{:?}", ids);

    Ok(ids)
}

async fn get_listing(
    web_driver: &WebDriver,
    url_id: String,
) -> Result<EraListingRaw, WebDriverError> {
    let _ = web_driver.get(&url_id).await;

    tokio::time::sleep(Duration::from_millis(500)).await;

    let description: Option<String> = match web_driver.find(By::Id("detail-description")).await {
        Ok(details_web_element) => Some(details_web_element.text().await?),
        Err(_) => {
            println!("Url {} did not have a description", url_id);
            None
        }
    };

    let details_vec: Vec<WebElement> = web_driver.find_all(By::ClassName("detail")).await?;

    let price: String = web_driver
        .find(By::ClassName("price-value"))
        .await?
        .inner_html()
        .await?;

    let mut details_split_by_string: Vec<String> = Vec::new();

    for div in details_vec {
        details_split_by_string.push(div.text().await?);
    }

    let era_listing_raw = EraListingRaw {
        price,
        description,
        details_split_by_string,
        url_id,
    };

    Ok(era_listing_raw)
}

pub async fn era_scrape_mechanism() -> Result<(), WebDriverError> {
    let web_driver: WebDriver = initialize_driver().await?;
    let era_ids_read: File = get_file_read("era_ids.txt").await?;
    let mut era_ids_write: File = get_file_write_append("era_ids.txt").await?;
    let mut era_write: File = get_file_write_append("era.json").await?;
    let era_ids: String = get_content_as_string(era_ids_read).await?;

    let mut latest_url_ids: Vec<String> = Vec::new();

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
                if latest_url_ids == url_ids {
                    println!(
                        "url_ids from page {} are equal to url_ids from previous page",
                        page
                    );
                    break;
                } else {
                    latest_url_ids = url_ids.clone();
                    for url_id in url_ids {
                        if !era_ids.contains(&url_id) {
                            println!("Url id: {}", &url_id);
                            match get_listing(&web_driver, url_id).await {
                                Ok(era_listing) => {
                                    write_to_file(
                                        &mut era_write,
                                        format!("{}\n", json!(era_listing)),
                                    )
                                    .await?;

                                    write_to_file(
                                        &mut era_ids_write,
                                        format!("{}\n", era_listing.url_id),
                                    )
                                    .await?;
                                }
                                Err(_) => println!("Failed to grab listing"),
                            };

                            tokio::time::sleep(Duration::from_millis(500)).await;
                        } else {
                            println!("Url id: {} is already scrapped", url_id);
                        }
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
    match era_scrape_mechanism().await {
        Ok(_) => {
            println!("Era scrapper mechanism finished")
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
