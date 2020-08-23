use serde_json::error::Error as serde_json_error;
use thiserror::Error;

pub type Result<T, E = GraphError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum GraphError {
    #[error("the data for key is not available")]
    InvalidGraph(#[from] serde_json_error),
}
