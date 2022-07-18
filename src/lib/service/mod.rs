pub mod action;
pub mod ask;

use crate::{ClipError, DataError};
use sqlx;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("database error: {0}")]
    Data(DataError),
    #[error("service error: {0}")]
    Clip(#[from] ClipError),
    #[error("not found")]
    NotFound,
    #[error("permission error: {0}")]
    PermissionError(String),
}

impl From<DataError> for ServiceError {
    fn from(err: DataError) -> Self {
        match err {
            DataError::DatabaseError(e) => match e {
                sqlx::Error::RowNotFound => Self::NotFound,
                _ => Self::Data(DataError::DatabaseError(e)),
            },
        }
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::NotFound,
            _ => Self::Data(DataError::DatabaseError(err)),
        }
    }
}
