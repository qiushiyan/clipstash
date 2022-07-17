use crate::domain::clip::ClipError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Content(String);

impl Content {
    pub fn new(content: &str) -> Result<Self, ClipError> {
        if content.is_empty() {
            return Err(ClipError::EmptyContent);
        }
        Ok(Content(content.to_owned()))
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
