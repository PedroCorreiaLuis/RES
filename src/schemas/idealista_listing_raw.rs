use crate::schemas::llm::ToLLMRequestBody;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct IdealistaListingRaw {
    pub price: String,
    pub description: Option<String>,
    pub details_split_by_string: String,
    pub url_id: String,
}

impl ToLLMRequestBody for IdealistaListingRaw {}
