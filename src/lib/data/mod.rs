use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use sqlx::Sqlite;
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

pub mod model;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Sqlite> {
    pub async fn new(uri: &str) -> Self {
        let pool = sqlx::sqlite::SqlitePoolOptions::new().connect(uri).await;

        match pool {
            Ok(pool) => Self(pool),
            Err(e) => {
                eprintln!("{:?}", e);
                println!("Use `sqlx database setup` to create new database");
                panic!("database error")
            }
        }
    }

    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}

pub type DatabasePool = sqlx::sqlite::SqlitePool;
pub type AppDatabase = Database<Sqlite>;
pub type Transaction<'a> = sqlx::Transaction<'a, Sqlite>;
pub type AppDatabaseRow = sqlx::sqlite::SqliteRow;
pub type AppQueryResult = sqlx::sqlite::SqliteQueryResult;

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
