use crate::schemas::llm::{
    to_llm_request_body_json, LLMMessageResponse, LLMMessageResponseRaw, LLMRealStateResponse,
    LLMResponse, LLMResponseChoice, LLMResponseRaw, ToLLMRequestBody,
};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::{Client, Response};
use serde::Serialize;
use serde_json::json;
use std::error::Error;
use std::string::ToString;

pub async fn call_real_estate_llm<T: ToLLMRequestBody + Serialize>(
    request: T,
    key: &str,
) -> reqwest::Result<LLMResponse> {
    let client: Client = Client::new();
    let mut headers_map: HeaderMap = HeaderMap::new();

    headers_map.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/json").unwrap(),
    );
    headers_map.insert(
        "Authorization",
        HeaderValue::from_str(format!("Bearer {}", key).as_str()).unwrap(),
    );

    let response: Response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .body(json!(request.to_llm_request_body()).to_string())
        .headers(headers_map)
        .send()
        .await?;

    let llm_response_raw = response.json::<LLMResponseRaw>().await?;

    let mut llm_response_choices: Vec<LLMResponseChoice> = Vec::new();

    for choice in llm_response_raw.choices {
        let llm_message_response_raw: LLMMessageResponseRaw = choice.message;
        let llm_message_response: LLMRealStateResponse =
            serde_json::from_str(&*llm_message_response_raw.content).unwrap();

        let message_response: LLMMessageResponse = LLMMessageResponse {
            role: llm_message_response_raw.role,
            content: llm_message_response,
        };
        llm_response_choices.push(LLMResponseChoice {
            logprobs: choice.logprobs,
            finish_reason: choice.finish_reason,
            index: choice.index,
            message: message_response,
            refusal: choice.refusal,
        })
    }

    Ok(LLMResponse {
        id: llm_response_raw.id,
        provider: llm_response_raw.provider,
        model: llm_response_raw.model,
        object: llm_response_raw.object,
        created: llm_response_raw.created,
        choices: llm_response_choices,
    })
}

pub async fn call_real_estate_llm_json(
    request: String,
    key: &str,
) -> Result<LLMResponse, Box<dyn Error>> {
    let client: Client = Client::new();
    let mut headers_map: HeaderMap = HeaderMap::new();

    headers_map.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/json").unwrap(),
    );
    headers_map.insert(
        "Authorization",
        HeaderValue::from_str(format!("Bearer {}", key).as_str()).unwrap(),
    );

    let response: Response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .body(json!(to_llm_request_body_json(request)).to_string())
        .headers(headers_map)
        .send()
        .await?;

    let llm_response_raw_text: String = response.text().await?;

    let llm_response_raw: LLMResponseRaw =
        serde_json::from_str::<LLMResponseRaw>(&llm_response_raw_text)?;

    let mut llm_response_choices: Vec<LLMResponseChoice> = Vec::new();

    for choice in llm_response_raw.choices {
        let llm_message_response_raw: LLMMessageResponseRaw = choice.message;
        let fixed_raw_content: String = if llm_message_response_raw.content.ends_with("}") {
            llm_message_response_raw.content
        } else {
            format!("{}{}", &*llm_message_response_raw.content, "}")
        };
        let llm_message_response: LLMRealStateResponse = serde_json::from_str(&*fixed_raw_content)?;

        let message_response: LLMMessageResponse = LLMMessageResponse {
            role: llm_message_response_raw.role,
            content: llm_message_response,
        };
        llm_response_choices.push(LLMResponseChoice {
            logprobs: choice.logprobs,
            finish_reason: choice.finish_reason,
            index: choice.index,
            message: message_response,
            refusal: choice.refusal,
        })
    }

    Ok(LLMResponse {
        id: llm_response_raw.id,
        provider: llm_response_raw.provider,
        model: llm_response_raw.model,
        object: llm_response_raw.object,
        created: llm_response_raw.created,
        choices: llm_response_choices,
    })
}

pub const FREE_LLAMA_MODEL: &str = "meta-llama/llama-3.2-3b-instruct:free";

pub const SYSTEM_ROLE: &str = "system";
pub const SYSTEM_CONTENT: &str = "You are a real estate guru.";

pub const USER_ROLE: &str = "user";
pub const USER_CONTENT: &str = "Based on the following JSON that I will give you in Portuguese from Portugal, only with a JSON with the following properties:
          - url_id
          - no_bedrooms
          - no_bathrooms
          - has_garage
          - has_pool
          - has_good_location
          - location
          - average_price
          - average_sqr_meters
          - average_price_per_sqr_meters
          - sqr_meters
          - price
          - summary
          - score

          The properties should be calculated following these instructions:
          - url_id is extracted from the provided JSON
          - no_bedrooms is extracted from the provided JSON, if its not a number cast it to an integer, not a String
          - no_bathrooms is extracted from the provided JSON, if its not a number cast it to an integer, not a String
          - has_garage is inferred from the provided JSON, if its not a bool cast it to a bool, not a String
          - has_pool is inferred from the provided JSON, if its not a bool cast it to a bool, not a String
          - has_good_location is inferred from the provided JSON, if its not a bool cast it to a bool, not a String
          - location is extracted from the provided JSON
          - average_price is inferred from the real estate market given the specific location in the output JSON without taking the provided JSON into consideration, if its not a number cast it to a number, not a String
          - average_sqr_meters is inferred from the real estate market given the specific location in the output JSON without taking the provided JSON into consideration, if its not a number cast it to a number, not a String
          - average_price_per_sqr_meters is calculated by dividing average_sqr_meters per average_price both in the output JSON, it should be the resulting number not a String
          - sqr_meters is extracted from the provided JSON, if its not a number cast it to a number, not a String
          - price is extracted from the provided JSON, if its not a number give null, not a String
          - summary is a summary of the description and details of the provided JSON, is a String that should not contain more thn 30 characters
          - score is an aggregation of all the features calculated in the JSON outputted excluding the score. It is bounded of a score of 1 being the worst deal possible, and the score of 10 being the deal of a life time. The type should be a float

           Always reply just with a valid Json in English and every information inside the JSON also English, nothing else.
           To be a valid JSON it needs finish with a }.
           Don't add the json block format.

          The provided JSON is :";
