use crate::llms::llm_utils::call_real_estate_llm_json;
use crate::schemas::llm::LLMResponse;
use crate::utils::file_utils::{
    get_content_lines, get_file_read, get_file_write_append, write_to_file,
};
use std::error::Error;
use std::time::Duration;
use tokio::fs::File;

async fn llm_mechanism(
    key: &str,
    input_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    let read_input: File = get_file_read(input_path).await?;
    let mut write_output: File = get_file_write_append(output_path).await?;

    let content_lines: Vec<String> = get_content_lines(read_input).await?;

    for content_line in content_lines {
        let llm_response: LLMResponse = call_real_estate_llm_json(content_line, key).await?;
        println!("LLM response id {}", llm_response.id);
        // Free models have a limit of 20/min and 200/day
        tokio::time::sleep(Duration::from_millis(3000)).await;
        write_to_file(
            &mut write_output,
            format!("{}\n", serde_json::to_string(&llm_response)?),
        )
        .await?;
    }

    Ok(())
}
pub async fn run(key: &str, input_path: &str, output_path: &str) {
    match llm_mechanism(key, input_path, output_path).await {
        Ok(_) => {
            println!("Era scrapper mechanism finished")
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
