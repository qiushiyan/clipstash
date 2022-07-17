use crate::data::DatabaseId;
use derive_more::{Constructor, From};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Constructor, From, Serialize, Deserialize)]
pub struct ClipId(DatabaseId);

impl ClipId {
    pub fn into_inner(self) -> DatabaseId {
        self.0
    }
}

impl Default for ClipId {
    fn default() -> Self {
        Self(DatabaseId::nil())
    }
}
