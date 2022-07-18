use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Constructor)]
pub struct Hits(i64);

impl Hits {
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
