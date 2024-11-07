use crate::llms::llm_utils::{
    FREE_LLAMA_MODEL, SYSTEM_CONTENT, SYSTEM_ROLE, USER_CONTENT, USER_ROLE,
};
use crate::schemas::llm::{LLMBodyMessage, LLMRequestBody, ToLLMRequestBody};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize)]
pub struct RemaxListingRaw {
    pub price: String,
    pub description: String,
    pub details_split_by_string: Vec<String>,
    pub url_id: String,
}

impl ToLLMRequestBody for RemaxListingRaw {
    fn to_llm_request_body(&self) -> LLMRequestBody {
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
