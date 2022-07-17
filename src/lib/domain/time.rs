use chrono::{DateTime, NaiveDateTime, Utc};
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Serialize, Deserialize, From)]
pub struct Time(DateTime<Utc>);

impl Time {
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }

    pub fn to_timestamp(&self) -> i64 {
        self.0.timestamp()
    }

    pub fn from_naive_utc(time: NaiveDateTime) -> Self {
        Self(DateTime::<Utc>::from_utc(time, Utc))
    }
}

impl FromStr for Time {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // yyyy-mm-dd
        Ok(format!("{}T00:00:00Z", s).parse::<DateTime<Utc>>()?.into())
    }
}
