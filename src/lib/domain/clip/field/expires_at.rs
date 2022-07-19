use crate::domain::time::Time;
use derive_more::From;
use rocket::form::{self, FromFormField, ValueField};
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

#[rocket::async_trait]
impl<'r> FromFormField<'r> for ExpiresAt {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        if field.value.trim().is_empty() {
            Ok(Self(None))
        } else {
            Ok(ExpiresAt::from_str(field.value)
                .map_err(|e| form::Error::validation(format!("{}", e)))?)
        }
    }
}
