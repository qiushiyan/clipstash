use crate::domain::time::Time;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Constructor)]
pub struct CreatedAt(Time);

impl CreatedAt {
    pub fn into_inner(self) -> Time {
        self.0
    }
}
