pub mod ctx;
pub mod form;
pub mod http;
pub mod renderer;

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
