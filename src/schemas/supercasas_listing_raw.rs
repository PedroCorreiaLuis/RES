use crate::schemas::llm::ToLLMRequestBody;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SuperCasasListingRaw {
    pub price: String,
    pub description: Option<String>,
    pub details_split_by_string: Vec<String>,
    pub url_id: String,
}

impl ToLLMRequestBody for SuperCasasListingRaw {}
