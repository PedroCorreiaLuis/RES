use crate::llms::llm_utils::{
    FREE_LLAMA_MODEL, SYSTEM_CONTENT, SYSTEM_ROLE, USER_CONTENT, USER_ROLE,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize)]
pub struct LLMRequestBody {
    pub model: String,
    pub messages: Vec<LLMBodyMessage>,
}

#[derive(Deserialize, Serialize)]
pub struct LLMBodyMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Serialize)]
pub struct LLMResponseRaw {
    pub id: String,
    pub provider: String,
    pub model: String,
    pub object: String,
    pub created: u32,
    pub choices: Vec<LLMResponseChoiceRaw>,
}

#[derive(Deserialize, Serialize)]
pub struct LLMResponse {
    pub id: String,
    pub provider: String,
    pub model: String,
    pub object: String,
    pub created: u32,
    pub choices: Vec<LLMResponseChoice>,
}

#[derive(Deserialize, Serialize)]
pub struct LLMResponseChoiceRaw {
    pub logprobs: Option<String>,
    pub finish_reason: String,
    pub index: u32,
    pub message: LLMMessageResponseRaw,
    pub refusal: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct LLMResponseChoice {
    pub logprobs: Option<String>,
    pub finish_reason: String,
    pub index: u32,
    pub message: LLMMessageResponse,
    pub refusal: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct LLMMessageResponseRaw {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Serialize)]
pub struct LLMMessageResponse {
    pub role: String,
    pub content: LLMRealStateResponse,
}

#[derive(Deserialize, Serialize)]
pub struct LLMRealStateResponse {
    url_id: String,
    no_bedrooms: u32,
    no_bathrooms: u32,
    has_garage: bool,
    has_pool: bool,
    has_good_location: bool,
    location: String,
    average_price: u32,
    average_sqr_meters: u32,
    average_price_per_sqr_meters: u32,
    sqr_meters: u32,
    price: u32,
    summary: String,
    score: u32,
}

pub trait ToLLMRequestBody {
    fn to_llm_request_body(&self) -> LLMRequestBody
    where
        Self: Serialize,
    {
        let mut message_vec = Vec::new();

        message_vec.push(LLMBodyMessage {
            role: SYSTEM_ROLE.to_string(),
            content: SYSTEM_CONTENT.to_string(),
        });

        message_vec.push(LLMBodyMessage {
            role: USER_ROLE.to_string(),
            content: format!("{}\n {}", USER_CONTENT, json!(self)),
        });

        LLMRequestBody {
            model: FREE_LLAMA_MODEL.to_string(),
            messages: message_vec,
        }
    }
}
