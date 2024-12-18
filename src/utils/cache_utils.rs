use crate::utils::file_utils::{get_content_lines, write_to_file};
use moka::future::Cache;
use std::error::Error;
use tokio::fs::File;

pub async fn spawn_cache(opt_file: Option<File>) -> Result<Cache<String, String>, Box<dyn Error>> {
    match opt_file {
        None => Ok(Cache::new(1000000)),
        Some(file) => {
            let mut cache: Cache<String, String> = Cache::new(1000000);
            let content_vec: Vec<String> = get_content_lines(file).await?;

            for line in content_vec {
                let split_line = line.split("|:|").collect::<Vec<&str>>();
                // println!("Split line {} , {}", split_line[0], split_line[1]);
                cache
                    .insert(split_line[0].to_string(), split_line[1].to_string())
                    .await;
            }
            Ok(cache)
        }
    }
}

pub async fn export(
    cache: &Cache<String, String>,
    file_writer: &mut File,
) -> Result<(), Box<dyn Error>> {
    cache.run_pending_tasks().await;
    println!("Exporting {} elements of the cache", cache.entry_count());
    for (key, value) in cache.iter() {
        write_to_file(file_writer, format!("{}|:|{}\n", key, value)).await?
    }
    Ok(())
}
