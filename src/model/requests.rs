use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddItemsToTableRequest {
    pub items_names: Vec<String>,
}
