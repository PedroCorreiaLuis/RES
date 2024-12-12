use serde_json::json;
use std::time::Duration;
use thirtyfour::{By, WebDriver, WebElement};

use crate::schemas::imovirtual_listing_raw::ImovirtualListingRaw;
use crate::scrappers::driver::initialize_driver;
use crate::scrappers::file_utils::*;
use thirtyfour::error::WebDriverError;
use tokio::fs::File;
use tokio::time::timeout;

async fn get_url_ids(web_driver: &WebDriver, page: u32) -> Result<Vec<String>, WebDriverError> {
    let _ = web_driver.get(format!("https://www.imovirtual.com/pt/resultados/comprar/apartamento/todo-o-pais?viewType=listing&by=LATEST&direction=DESC&page={}",page)).await;
    tokio::time::sleep(Duration::from_millis(2500)).await;

    let mut ids: Vec<String> = Vec::new();

    // The site takes some time to load this information
    while ids.is_empty() {
        let parent_divs = web_driver
            .find_all(By::Css("[data-cy='listing-item-link']"))
            .await?;

        for div in parent_divs {
            ids.push(div.attr("href").await?.unwrap());
        }
    }

    println!("{:?}", ids);

    Ok(ids)
}

async fn get_listing(
    web_driver: &WebDriver,
    url_id: String,
) -> Result<ImovirtualListingRaw, WebDriverError> {
    let _ = web_driver
        .get(format!("https://www.imovirtual.pt/{}", url_id))
        .await;

    tokio::time::sleep(Duration::from_millis(200)).await;

    let description: String = web_driver
        .find(By::Css("[data-cy='adPageAdDescription']"))
        .await?
        .text()
        .await?;

    let details_vec: Vec<WebElement> = web_driver.find_all(By::ClassName("e15n0fyo2")).await?;

    let price: Option<String> = match web_driver
        .find(By::Css("[data-cy='adPageHeaderPrice']"))
        .await
    {
        Ok(element) => Some(element.text().await?),
        Err(_) => None,
    };

    let mut details_split_by_string: Vec<String> = Vec::new();

    for div in details_vec {
        details_split_by_string.push(div.text().await?);
    }

    let imovirtual_listing_raw: ImovirtualListingRaw = ImovirtualListingRaw {
        price,
        description,
        details_split_by_string,
        url_id,
    };

    Ok(imovirtual_listing_raw)
}

pub async fn imovirtual_scrape_mechanism() -> Result<(), WebDriverError> {
    let web_driver: WebDriver = initialize_driver().await?;
    let imovirtual_ids_read: File = get_file_read("imovirtual_ids.txt").await?;
    let mut imovirtual_ids_write: File = get_file_write("imovirtual_ids.txt").await?;
    let mut imovirtual_write: File = get_file_write("imovirtual.json").await?;
    let imovirtual_ids: String = get_content_as_string(imovirtual_ids_read).await?;

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

        let elements_found: bool = match web_driver
            .find(By::Css("[data-cy='no-search-results']"))
            .await
        {
            Ok(element) => match element.text().await {
                Ok(text) => !text.contains("Nenhum resultado encontrado"),
                Err(_) => true,
            },
            Err(_) => true,
        };

        if elements_found {
            match url_ids_vec {
                Ok(url_ids) => {
                    for url_id in url_ids {
                        if !imovirtual_ids.contains(&url_id) {
                            println!("Url id: {}", url_id);
                            let imovirtual_listing: ImovirtualListingRaw =
                                get_listing(&web_driver, url_id).await?;

                            write_to_file(
                                &mut imovirtual_write,
                                format!("{}\n", json!(imovirtual_listing)),
                            )
                            .await?;

                            write_to_file(
                                &mut imovirtual_ids_write,
                                format!("{}\n", imovirtual_listing.url_id),
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
            println!("No more pages after {}", page);
            break;
        }
    }

    Ok(())
}

pub async fn run() {
    match imovirtual_scrape_mechanism().await {
        Ok(_) => {
            println!("imovirtual scrapper mechanism finished")
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
