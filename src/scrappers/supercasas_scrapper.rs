use serde_json::json;
use std::time::Duration;
use thirtyfour::{By, WebDriver, WebElement};

use crate::schemas::supercasas_listing_raw::SuperCasasListingRaw;
use crate::scrappers::driver::initialize_driver;
use crate::scrappers::scrapper_utils::PORTUGUESE_DISTRICTS;
use crate::utils::file_utils::*;
use thirtyfour::error::WebDriverError;
use tokio::fs::File;
use tokio::time::timeout;
use tokio_retry::strategy::FixedInterval;
use tokio_retry::Retry;

async fn get_url_ids(
    web_driver: &WebDriver,
    page: u32,
    district: &str,
) -> Result<Vec<String>, WebDriverError> {
    let _ = web_driver
        .get(format!(
            "https://supercasa.pt/comprar-casas/{}-distrito/pagina-{}?ordem=atualizado-desc",
            district, page
        ))
        .await;

    tokio::time::sleep(Duration::from_millis(2500)).await;

    let mut ids: Vec<String> = Vec::new();

    // The site takes some time to load this information
    while ids.is_empty() {
        let parent_div: Vec<WebElement> = web_driver
            .find_all(By::ClassName("property-list-title"))
            .await?;

        // Find all divs within it and get IDs

        for div in parent_div {
            let child_divs: WebElement = div.find(By::Tag("a")).await?;
            ids.push(child_divs.attr("href").await?.unwrap());
        }
    }

    println!("{:?}", ids);

    Ok(ids)
}

async fn get_listing(
    web_driver: &WebDriver,
    url_id: String,
) -> Result<SuperCasasListingRaw, WebDriverError> {
    let _ = web_driver
        .get(format!("https://supercasa.pt{}", url_id))
        .await;

    tokio::time::sleep(Duration::from_millis(200)).await;

    let description: Option<String> = match web_driver
        .find(By::ClassName("detail-info-description-txt"))
        .await
    {
        Ok(details_web_element) => Some(details_web_element.text().await?),
        Err(_) => {
            println!("Url {} did not have a description", url_id);
            None
        }
    };
    let details_vec = web_driver
        .find_all(By::ClassName("detail-info-features-list"))
        .await?;

    let price: String = web_driver
        .find(By::ClassName("property-price"))
        .await?
        .find(By::Tag("span"))
        .await?
        .text()
        .await?;

    let mut details_split_by_string: Vec<String> = Vec::new();

    for div in details_vec {
        details_split_by_string.push(div.text().await?);
    }

    let supercasas_listing_raw = SuperCasasListingRaw {
        price,
        description,
        details_split_by_string,
        url_id,
    };

    Ok(supercasas_listing_raw)
}

pub async fn supercasas_scrape_mechanism() -> Result<(), WebDriverError> {
    let web_driver: WebDriver = initialize_driver().await?;
    let supercasas_ids_read: File = get_file_read("supercasas_ids.txt").await?;
    let mut supercasas_ids_write: File = get_file_write_append("supercasas_ids.txt").await?;
    let mut supercasas_write: File = get_file_write_append("supercasas.json").await?;
    let supercasas_ids: String = get_content_as_string(supercasas_ids_read).await?;

    for district in PORTUGUESE_DISTRICTS {
        for page in 1.. {
            println!("\nScrapper mechanism page {}", page);

            // If we cannot get the page loaded in 30 seconds we ignore it and move on
            let url_ids_vec: Result<Vec<String>, WebDriverError> = timeout(
                Duration::from_secs(30),
                get_url_ids(&web_driver, page, district),
            )
            .await
            .unwrap_or_else(|_| {
                println!("Scrapper mechanism url_ids timed out after 30 seconds!");
                Ok(Vec::new())
            });

            let current_page: String = web_driver.current_url().await?.to_string();
            let elements_found: bool = match web_driver
                .find(By::ClassName("home-search-content"))
                .await
            {
                Ok(element) => match element.text().await {
                    Ok(text) => !text.contains("Não encontrámos imóveis para o que procuras..."),
                    Err(_) => true,
                },
                Err(_) => true,
            };

            if current_page.contains(district)
                && current_page.contains(page.to_string().as_str())
                && elements_found
            {
                match url_ids_vec {
                    Ok(url_ids) => {
                        for url_id in url_ids {
                            if !supercasas_ids.contains(&url_id) {
                                println!("Url id: {}", url_id);

                                let supercasas_listing: SuperCasasListingRaw = Retry::spawn(
                                    FixedInterval::from_millis(500).take(6),
                                    || async { get_listing(&web_driver, url_id.clone()).await },
                                )
                                .await?;

                                write_to_file(
                                    &mut supercasas_write,
                                    format!("{}\n", json!(supercasas_listing)),
                                )
                                .await?;

                                write_to_file(
                                    &mut supercasas_ids_write,
                                    format!("{}\n", supercasas_listing.url_id),
                                )
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
            } else {
                println!("No more pages after {} for district {}", page, district);
                break;
            }
        }
    }

    Ok(())
}

pub async fn run() {
    match supercasas_scrape_mechanism().await {
        Ok(_) => {
            println!("SuperCasas scrapper mechanism finished")
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
