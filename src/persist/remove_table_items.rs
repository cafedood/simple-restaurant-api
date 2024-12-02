use actix_request_identifier::RequestId;
use log::{debug, error};
use mysql::prelude::*;
use mysql::{Pool};
use crate::common::errors::PersistenceError;
use crate::model::responses::{RemoveTableItemResponse};

pub fn remove_table_item(
    pool: &Pool,
    request_id: RequestId,
    table_number: u32,
    item_id: u32,
) -> Result<RemoveTableItemResponse, PersistenceError> {
    let query = generate_query();

    let mut conn = pool.get_conn().map_err(|_| PersistenceError::DBConnError)?;

    conn.query_drop("START TRANSACTION").map_err(|_| PersistenceError::TransactionStartError)?;
    match conn.exec_drop(query, (table_number, item_id,)) {
        Ok(_) => {
            let affected_rows = conn.affected_rows();
            conn.query_drop("COMMIT").map_err(|_| PersistenceError::CommitError)?;
            if affected_rows > 0 {
                Ok(generate_success_response(item_id))
            } else {
                Ok(generate_absent_response(item_id))
            }
        }
        Err(e) => {
            conn.query_drop("ROLLBACK").map_err(|_| PersistenceError::RollbackError)?;
            error!("{:?}", e);
            Ok(generate_failed_response())
        }
    }
}

fn generate_absent_response(item_id: u32) -> RemoveTableItemResponse {
    RemoveTableItemResponse {
        status: "success".to_string(),
        message: format!("No item with id {}", item_id),
    }
}

fn generate_failed_response() -> RemoveTableItemResponse {
    RemoveTableItemResponse {
        status: "failed".to_string(),
        message: "Can NOT remove desired table item".to_string(),
    }
}

fn generate_success_response(item_id: u32) -> RemoveTableItemResponse {
    RemoveTableItemResponse {
        status: "success".to_string(),
        message: format!("Removed item: {}", item_id),
    }
}

fn generate_query() -> String {
    "DELETE FROM table_items WHERE table_number = ? and item_id = ?".to_string()
}