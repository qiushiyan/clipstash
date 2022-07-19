use crate::domain::clip::field;
use rocket::form::FromForm;
use serde::Serialize;

#[derive(Debug, Serialize, FromForm)]
pub struct NewClip {
    pub title: field::Title,
    pub content: field::Content,
    pub password: field::Password,
    pub expires_at: field::ExpiresAt,
}

#[derive(Debug, Serialize, FromForm)]
pub struct GetPasswordProtectedClip {
    pub password: field::Password,
}
