use lambda_runtime::error::HandlerError;

use log::SetLoggerError;
use serde_json::error::Error as serde_json_error;
use thiserror::Error;

pub type Result<T, E = GraphError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum GraphError {
    #[error("AWS HandlerError!")]
    AWSError(#[from] HandlerError),
    #[error("logger error!")]
    LoggerError(#[from] SetLoggerError),
    #[error("Unable to deserialize data from JSON file")]
    SerdeError(#[from] serde_json_error),
}
