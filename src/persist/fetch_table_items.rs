use actix_request_identifier::RequestId;
use log::error;
use mysql::prelude::*;
use mysql::{from_value, Pool, Value, Row};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::common::errors::MysqlValueError::{MissingDatetime, MissingInteger, MissingString};
use crate::common::errors::{generate_mysql_value_error, PersistenceError};
use crate::model::resources::TableItem;

pub fn get_table_items(
    pool: &Pool,
    request_id: RequestId,
    table_number: u32,
    items_ids: Option<Vec<u32>>,
    items_names: Option<Vec<String>>,
) -> Result<Vec<TableItem>, PersistenceError> {
    let (query, params) = generate_query_and_params(table_number, items_ids, items_names);

    let mut conn = pool.get_conn().map_err(|_| PersistenceError::DBConnError)?;
    let result = match conn.exec_iter(query, params) {
        Ok(result) => {
            let table_items: Vec<TableItem> = result
                .map(|row| convert_row_to_table_item(row))
                .filter_map(Result::ok)
                .collect();

            if table_items.is_empty() {
                error!("Items NOT found, request ID: {request_id}");
                Err(PersistenceError::ResourceNotFound)
            } else {
                Ok(table_items)
            }
        }
        Err(e) => {
            error!("DB retriving failed for {request_id}: {:?}", e);
            Err(PersistenceError::DBOpError)
        }
    };
    result
}

fn generate_query_and_params(table_number: u32, items_ids: Option<Vec<u32>>,
                             items_names: Option<Vec<String>>,
) -> (String, Vec<Value>) {
    let mut query = String::from("SELECT * FROM table_items WHERE table_number = ?");
    let mut params: Vec<Value> = vec![Value::from(table_number)];
    let mut conditions = Vec::new();

    if let Some(ref items_ids) = items_ids {
        if !items_ids.is_empty() {
            conditions.push(format!("item_id IN ({})", vec!["?"; items_ids.len()].join(",")));
            params.extend(items_ids.iter().map(|&id| Value::from(id)));
        }
    }

    if let Some(ref items_names) = items_names {
        if !items_names.is_empty() {
            conditions.push(format!("item_name IN ({})", vec!["?"; items_names.len()].join(",")));
            params.extend(items_names.iter().map(|name| Value::from(name)));
        }
    }

    if !conditions.is_empty() {
        query.push_str(" AND ");
        query.push_str(&format!("( {} )", conditions.join(" OR ")));
    }
    (query, params)
}

fn convert_row_to_table_item(row: Result<Row, mysql::Error>) -> Result<TableItem, mysql::Error> {
    let row = row?;
    let item_id: u32 = row.get(0)
        .ok_or_else(|| generate_mysql_value_error(MissingInteger, "item_id".into()))?;

    let table_number: u32 = row.get(1)
        .ok_or_else(|| generate_mysql_value_error(MissingInteger, "table_number".into()))?;

    let item_name: String = match row.get(2) {
        Some(Value::Bytes(bytes)) => String::from_utf8_lossy(&*bytes).into_owned(),
        Some(other) => from_value(other),
        None => return Err(generate_mysql_value_error(MissingString, "item_name".into())),
    };

    let prepare_minutes: u32 = row.get(3)
        .ok_or_else(|| generate_mysql_value_error(MissingInteger, "prepare_minutes".into()))?;

    let ordered_on: String = match row.get(4) {
        Some(Value::Date(year, month, day, hour, minute, second, _micro_second)) => {
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).unwrap(),
                NaiveTime::from_hms_opt(hour as u32, minute as u32, second as u32).unwrap(),
            ).format("%Y-%m-%d %H:%M:%S").to_string()
        }
        Some(other) => from_value(other),
        None => return Err(generate_mysql_value_error(MissingDatetime, "ordered_on".into())),
    };

    Ok(TableItem {
        item_id,
        table_number,
        item_name,
        prepare_minutes,
        ordered_on,
    })
}