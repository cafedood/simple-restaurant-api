use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error, From};
use log::{error};
use serde::Serialize;
use mysql::Error;

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    DBConnError,
    TransactionStartError,
    CommitError,
    RollbackError,
    ResourceNotFound,
    DBOpError,
}

#[derive(Debug, Display, Error, Serialize)]
pub struct PersistenceErrorResponse {
    message: String,
}

impl ResponseError for PersistenceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            PersistenceError::DBConnError => {
                let msg = "No connections available from pool";
                error!("{}", msg);
                HttpResponse::InternalServerError().json(PersistenceErrorResponse { message: msg.into() })
            }

            PersistenceError::TransactionStartError => {
                let msg = "Can NOT begin the transaction";
                error!("{}", msg);
                HttpResponse::InternalServerError().json(PersistenceErrorResponse { message: msg.into() })
            }

            PersistenceError::CommitError => {
                let msg = "Can NOT commit the transaction";
                error!("{}", msg);
                HttpResponse::InternalServerError().json(PersistenceErrorResponse { message: msg.into() })
            }

            PersistenceError::RollbackError => {
                let msg = "Can NOT rollback the transaction";
                error!("{}", msg);
                HttpResponse::InternalServerError().json(PersistenceErrorResponse { message: msg.into() })
            }

            PersistenceError::ResourceNotFound => {
                let msg = "Can NOT found the resource";
                error!("{}", msg);
                HttpResponse::NotFound().json(PersistenceErrorResponse { message: msg.into() })
            }

            PersistenceError::DBOpError => {
                let msg = "Can NOT Execute SQL";
                error!("{}", msg);
                HttpResponse::InternalServerError().json(PersistenceErrorResponse { message: msg.into() })
            }
        }
    }
}


#[derive(Debug, Display, Error, From)]
pub enum MysqlValueError {
    MissingString,
    MissingInteger,
    MissingDatetime
}
pub fn generate_mysql_value_error(err_type: MysqlValueError, column_name: String) -> Error {
    Error::MySqlError(mysql::MySqlError {
        state: "failed".to_string(),
        code: match err_type {
            MysqlValueError::MissingString => 1,
            MysqlValueError::MissingInteger => 2,
            MysqlValueError::MissingDatetime => 3
        },
        message: format!("Error: Issue with value existing in column({column_name})"),
    })
}