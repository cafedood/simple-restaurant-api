use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddItemsResponse {
    pub status: String,
    pub message: String,
    pub items_ids: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveTableItemResponse {
    pub status: String,
    pub message: String
}