use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Custom extension for each NFT
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct HashedBatchExtension {
    pub batch_date: String,           // формат: YYYY-MM-DD
    pub hashes: Vec<String>,         // список SHA256-хешів
}
