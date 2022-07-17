use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone, Debug, From, Display, Serialize, Deserialize)]
pub struct DatabaseId(Uuid);

impl DatabaseId {
    pub fn new() -> Self {
        Uuid::new_v4().into()
    }

    pub fn nil() -> Self {
        Self(Uuid::nil())
    }
}

impl Default for DatabaseId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DatabaseId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Uuid::parse_str(s)?.into())
    }
}
