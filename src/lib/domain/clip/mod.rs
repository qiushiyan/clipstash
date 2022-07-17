pub mod field;

use chrono;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid;

#[derive(Debug, Error)]
pub enum ClipError {
    #[error("invalid password, {0}")]
    InvalidPassword(String),
    #[error("invalid title, {0}")]
    InvalidTitle(String),
    #[error("invalid date, {0}")]
    InvalidDate(String),
    #[error("empty content")]
    EmptyContent,
    #[error("date parse error, {0}")]
    DateParseError(#[from] chrono::ParseError),
    #[error("id parse error, {0}")]
    InvalidId(#[from] uuid::Error),
    #[error("hits parse error, {0}")]
    InvalidHits(#[from] std::num::TryFromIntError),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Clip {
    pub id: field::ClipId,
    pub title: field::Title,
    pub content: field::Content,
    pub shortcode: field::ShortCode,
    pub created_at: field::CreatedAt,
    pub expires_at: field::ExpiresAt,
    pub password: field::Password,
    pub hits: field::Hits,
}
