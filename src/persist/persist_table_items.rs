use actix_request_identifier::RequestId;
use chrono::Local;
use log::{debug, error};
use mysql::prelude::*;
use mysql::{Pool};
use rand::Rng;
use crate::common::errors::PersistenceError;
use crate::model::responses::AddItemsResponse;

pub fn add_items_to_table(
    pool: &Pool,
    request_id: RequestId,
    table_number: u32,
    items_names: Vec<String>,
) -> Result<AddItemsResponse, PersistenceError> {
    let records: Vec<(String, String, String)> = generate_table_item_records(items_names);
    let query = generate_query(records.len());

    let mut conn = pool.get_conn().map_err(|_| PersistenceError::DBConnError)?;
    conn.query_drop("START TRANSACTION").map_err(|_| PersistenceError::TransactionStartError)?;

    let params = records.iter().flat_map(|(item_name, ordered_on, prepare_minutes)| {
        vec![
            table_number.into(),
            item_name.into(),
            ordered_on.into(),
            prepare_minutes.into(),
        ]
    }).collect::<Vec<mysql::Value>>();

    match conn.exec_drop(query, params) {
        Ok(_) => {
            let last_id = conn.query_first::<u32, _>("SELECT LAST_INSERT_ID()")
                .unwrap_or(Some(0)).expect("Error: Unable to get the inserted id");
            let item_ids: Vec<u32> = (last_id..last_id + records.len() as u32).collect();
            conn.query_drop("COMMIT").map_err(|_| PersistenceError::CommitError)?;
            Ok(generate_success_response(table_number, records.len(), item_ids))
        }
        Err(e) => {
            conn.query_drop("ROLLBACK").map_err(|_| PersistenceError::RollbackError)?;
            error!("{:?}", e);
            Ok(generate_failed_response(table_number))
        }
    }
}

fn generate_failed_response(table_number: u32) -> AddItemsResponse {
    AddItemsResponse {
        status: "failed".to_string(),
        message: format!("Can NOT add item(s) to table {}", table_number),
        items_ids: [].to_vec(),
    }
}

fn generate_success_response(table_number: u32, num_items: usize, item_ids: Vec<u32>) -> AddItemsResponse {
    AddItemsResponse {
        status: "success".to_string(),
        message: format!("Added {} items on table {}", num_items, table_number),
        items_ids: item_ids,
    }
}

fn generate_query(num_records: usize) -> String {
    let placeholders: Vec<String> = (0..num_records)
        .map(|_| "(?, ?, ?, ?)".to_string())
        .collect();
    format!(
        "INSERT INTO table_items (table_number, item_name, ordered_on, prepare_minutes) VALUES {}",
        placeholders.join(", ")
    )
}

fn generate_table_item_records(items_names: Vec<String>) -> Vec<(String, String, String)> {
    items_names.into_iter().filter_map(|item_name| {
        if item_name.replace(' ', "").trim().is_empty() {
            return None;
        }
        let ordered_on = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let prepare_minutes = rand::thread_rng().gen_range(5..16).to_string();
        Some((item_name, ordered_on, prepare_minutes))
    }).collect()
}
