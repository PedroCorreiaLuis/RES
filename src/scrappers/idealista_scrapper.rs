use rand::prelude::ThreadRng;
use rand::Rng;
use serde_json::json;
use std::time::Duration;
use thirtyfour::{By, WebDriver, WebElement};

use crate::schemas::idealista_listing_raw::IdealistaListingRaw;
use crate::scrappers::driver::initialize_driver;
use crate::scrappers::scrapper_utils::PORTUGUESE_DISTRICTS;
use crate::utils::file_utils::*;
use thirtyfour::error::WebDriverError;
use tokio::fs::File;
use tokio::time::timeout;
use tokio_retry::strategy::ExponentialBackoff;
use tokio_retry::Retry;

async fn get_url_ids(
    web_driver: &WebDriver,
    page: u32,
    district: &str,
) -> Result<Vec<String>, WebDriverError> {
    let _ = web_driver
        .get(format!(
            "https://www.idealista.pt/comprar-casas/{}-distrito/pagina-{}?ordem=atualizado-desc",
            district, page
        ))
        .await;

    let mut rng: ThreadRng = rand::thread_rng();
    let waiting_time: u64 = rng.gen_range(5000..=15000);

    tokio::time::sleep(Duration::from_millis(waiting_time)).await;

    let mut ids: Vec<String> = Vec::new();

    // The site takes some time to load this information
    while ids.is_empty() {
        let parent_divs: Vec<WebElement> =
            web_driver.find_all(By::Css("a[href^='/imovel']")).await?;

        for div in parent_divs {
            // Find all divs within it and get IDs
            ids.push(div.attr("href").await?.unwrap());
        }
    }

    println!("{:?}", ids);

    Ok(ids)
}

async fn get_listing(
    web_driver: &WebDriver,
    url_id: String,
) -> Result<IdealistaListingRaw, WebDriverError> {
    let _ = web_driver
        .get(format!("https://www.idealista.pt/{}", url_id))
        .await;

    let mut rng: ThreadRng = rand::thread_rng();
    let waiting_time: u64 = rng.gen_range(5000..=15000);

    tokio::time::sleep(Duration::from_millis(waiting_time)).await;

    let description: Option<String> = match web_driver.find(By::ClassName("comment")).await {
        Ok(element) => Some(element.text().await?),
        Err(_) => None,
    };

    let details_split_by_string: String = web_driver
        .find(By::ClassName("details-property"))
        .await?
        .text()
        .await?;

    let price: String = web_driver
        .find(By::ClassName("info-data-price"))
        .await?
        .text()
        .await?;

    let idealista_listing_raw: IdealistaListingRaw = IdealistaListingRaw {
        price,
        description,
        details_split_by_string,
        url_id,
    };

    Ok(idealista_listing_raw)
}

pub async fn idealista_scrape_mechanism() -> Result<(), WebDriverError> {
    let web_driver: WebDriver = initialize_driver().await?;
    let idealista_ids_read: File = get_file_read("idealista_ids.txt").await?;
    let mut idealista_ids_write: File = get_file_write_append("idealista_ids.txt").await?;
    let mut idealista_write: File = get_file_write_append("idealista.json").await?;
    let idealista_cache_read: File = get_file_read("idealista_cache.txt").await?;
    let idealista_cache: Vec<String> = get_content_lines(idealista_cache_read).await?;
    let idealista_ids: String = get_content_as_string(idealista_ids_read).await?;

    let cached_district: &str = match idealista_cache.first() {
        None => PORTUGUESE_DISTRICTS[0],
        Some(value) => value,
    };

    async fn cached_page() -> u32 {
        let idealista_cache_read: File = get_file_read("idealista_cache.txt").await.unwrap();

        match get_content_lines(idealista_cache_read)
            .await
            .unwrap()
            .last()
        {
            None => 1,
            Some(value) => value.parse::<u32>().unwrap(),
        }
    }

    for district in PORTUGUESE_DISTRICTS
        .iter()
        .skip_while(|&item| *item != cached_district)
    {
        for page in cached_page().await.. {
            println!("\nScrapper mechanism page {}", page);
            let mut idealista_cache_write_truncate: File =
                get_file_write_truncate("idealista_cache.txt").await?;

            write_to_file(
                &mut idealista_cache_write_truncate,
                format!("{}\n{}", district, page),
            )
            .await?;

            // If we cannot get the page loaded in 30 seconds we ignore it and move on
            let url_ids_vec: Result<Vec<String>, WebDriverError> = timeout(
                Duration::from_secs(30),
                Retry::spawn(
                    ExponentialBackoff::from_millis(500)
                        .max_delay(Duration::from_secs(30))
                        .take(2),
                    || async { get_url_ids(&web_driver, page, district).await },
                ),
            )
            .await
            .unwrap_or_else(|_| {
                println!("Scrapper mechanism url_ids timed out after 30 seconds!");
                Ok(Vec::new())
            });

            let selected_page: String = web_driver
                .find(By::Css("li.selected span"))
                .await?
                .text()
                .await?;

            println!("\nSelected page {}", selected_page);

            if selected_page == page.to_string() {
                match url_ids_vec {
                    Ok(url_ids) => {
                        for url_id in url_ids {
                            if !idealista_ids.contains(&url_id) {
                                println!("Url id: {}", url_id);
                                let idealista_listing: IdealistaListingRaw = Retry::spawn(
                                    ExponentialBackoff::from_millis(500)
                                        .max_delay(Duration::from_secs(30))
                                        .take(3),
                                    || async { get_listing(&web_driver, url_id.clone()).await },
                                )
                                .await?;

                                write_to_file(
                                    &mut idealista_write,
                                    format!("{}\n", json!(idealista_listing)),
                                )
                                .await?;

                                write_to_file(
                                    &mut idealista_ids_write,
                                    format!("{}\n", idealista_listing.url_id),
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
                let mut idealista_cache_write_truncate: File =
                    get_file_write_truncate("idealista_cache.txt").await?;

                write_to_file(
                    &mut idealista_cache_write_truncate,
                    format!("{}\n{}", district, 1),
                )
                .await?;
                break;
            }
        }
    }

    Ok(())
}

pub async fn run() {
    match Retry::spawn(
        ExponentialBackoff::from_millis(500)
            .max_delay(Duration::from_secs(30))
            .take(20),
        || async {
            println!("Retrying scrapper mechanism");
            idealista_scrape_mechanism().await
        },
    )
    .await
    {
        Ok(_) => {
            println!("Idealista scrapper mechanism finished");
            println!("Clearing cache");
            get_file_write_truncate("idealista_cache.txt")
                .await
                .unwrap();
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
