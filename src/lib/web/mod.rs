pub mod api;
pub mod ctx;
pub mod form;
pub mod hit_counter;
pub mod http;
pub mod renderer;

pub use api::ApiKey;
pub use hit_counter::HitCounter;

use rocket;

pub const PASSWORD_COOKIE: &str = "password";

#[derive(rocket::Responder)]
pub enum PageError {
    #[response(status = 500)]
    SerializationError(String),
    #[response(status = 500)]
    RenderError(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 500)]
    InternalError(String),
}

impl From<handlebars::RenderError> for PageError {
    fn from(err: handlebars::RenderError) -> Self {
        PageError::RenderError(format!("{}", err))
    }
}

impl From<serde_json::Error> for PageError {
    fn from(err: serde_json::Error) -> Self {
        PageError::SerializationError(format!("{}", err))
    }
}

pub mod test_helpers {
    use crate::data::{AppDatabase, Database};
    use sqlx::migrate::Migrator;
    use std::path::Path;
    use tokio::runtime::Handle;

    pub fn new_db(handle: &Handle) -> AppDatabase {
        handle.block_on(async move {
            let db = Database::new(":memory:").await;
            let migrator = Migrator::new(Path::new("./migrations")).await.unwrap();
            let pool = db.get_pool();
            migrator.run(pool).await.unwrap();
            db
        })
    }
}

#[cfg(test)]
pub mod test {
    use super::test_helpers::*;
    use crate::RocketConfig;
    use rocket::local::blocking::Client;
    pub fn config() -> RocketConfig {
        use crate::web::{hit_counter::HitCounter, renderer::Renderer};
        let rt = tokio::runtime::Runtime::new().expect("failed to spawn tokio runtime");
        let renderer = Renderer::new("templates/".into());
        let database = new_db(rt.handle());
        let maintenance = crate::domain::maintenance::Maintenance::spawn(
            database.get_pool().clone(),
            rt.handle().clone(),
        );
        let hit_counter = HitCounter::new(database.get_pool().clone(), rt.handle().clone());

        RocketConfig {
            renderer,
            database,
            hit_counter,
            maintenance,
        }
    }

    pub fn client() -> Client {
        let config = config();
        Client::tracked(crate::rocket(config)).expect("failed to build rocket instance")
    }
}
