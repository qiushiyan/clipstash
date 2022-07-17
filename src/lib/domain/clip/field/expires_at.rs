use crate::domain::time::Time;
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Serialize, Deserialize, From)]
pub struct ExpiresAt(Option<Time>);

impl ExpiresAt {
    pub fn new<T: Into<Option<Time>>>(expires_at: T) -> Self {
        let expires_at: Option<Time> = expires_at.into();
        Self(expires_at)
    }

    pub fn into_inner(self) -> Option<Time> {
        self.0
    }
}

impl Default for ExpiresAt {
    fn default() -> Self {
        Self(None)
    }
}

impl FromStr for ExpiresAt {
    type Err = crate::domain::clip::ClipError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Self(None))
        } else {
            Ok(Self(Some(Time::from_str(s)?)))
        }
    }
}
