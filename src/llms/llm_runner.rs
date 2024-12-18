use crate::llms::llm_utils::call_real_estate_llm_json;
use crate::schemas::llm::LLMResponse;
use crate::utils::cache_utils::{export, spawn_cache};
use crate::utils::file_utils::{
    get_content_lines, get_file_read, get_file_write_truncate, write_to_file,
};
use moka::future::Cache;
use std::error::Error;
use std::time::Duration;
use tokio::fs::File;

async fn llm_mechanism(
    key: &str,
    input_path: &str,
    output_path: &str,
    cache: &Cache<String, String>,
) -> Result<(), Box<dyn Error>> {
    let read_input: File = get_file_read(input_path).await?;
    let mut write_output: File = get_file_write_truncate(output_path).await?;

    let content_lines: Vec<String> = get_content_lines(read_input).await?;

    for content_line in content_lines {
        let content: String = match cache.get(&content_line).await {
            None => {
                println!("Cache miss");
                let llm_response: LLMResponse =
                    call_real_estate_llm_json(content_line.clone(), key).await?;
                // let key: &String = &llm_response.choices.first().unwrap().message.content.url_id;
                println!("LLM response id {}", llm_response.id);
                let llm_response_json: String = serde_json::to_string(&llm_response)?;
                // Free models have a limit of 20/min and 200/day
                //TODO add a proper key
                cache.insert(content_line, llm_response_json.clone()).await;
                tokio::time::sleep(Duration::from_millis(3000)).await;
                llm_response_json
            }
            Some(content) => {
                println!("Cache hit");
                content
            }
        };

        write_to_file(&mut write_output, format!("{}\n", content)).await?;
    }

    Ok(())
}
pub async fn run(key: &str, input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let cache_path: &str = "llm_cache.txt";
    let llm_cache_file_read: File = get_file_read(cache_path).await.unwrap();
    let cache: Cache<String, String> = spawn_cache(Some(llm_cache_file_read)).await?;

    match llm_mechanism(key, input_path, output_path, &cache).await {
        Ok(_) => {
            let mut llm_cache_file_writer: File = get_file_write_truncate(cache_path).await?;
            export(&cache, &mut llm_cache_file_writer).await?;

            Ok(println!("LLM mechanism finished"))
        }
        Err(e) => {
            let mut llm_cache_file_writer: File = get_file_write_truncate(cache_path).await?;
            export(&cache, &mut llm_cache_file_writer).await?;

            println!("Error: {:?}", e);
            panic!()
        }
    }
}
