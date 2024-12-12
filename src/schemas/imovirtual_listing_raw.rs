use crate::schemas::llm::ToLLMRequestBody;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ImovirtualListingRaw {
    pub price: Option<String>,
    pub description: String,
    pub details_split_by_string: Vec<String>,
    pub url_id: String,
}

impl ToLLMRequestBody for ImovirtualListingRaw {}
