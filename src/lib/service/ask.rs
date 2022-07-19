use derive_more::Constructor;
use serde::{Deserialize, Serialize};

use crate::domain::clip::field;
use crate::ShortCode;

#[derive(Debug, Deserialize, Serialize, Constructor)]
pub struct GetClip {
    pub shortcode: ShortCode,
    pub password: field::Password,
}

impl GetClip {
    pub fn from_str(shortcode: &str) -> Self {
        Self {
            shortcode: ShortCode::from(shortcode),
            password: field::Password::default(),
        }
    }
}

impl From<ShortCode> for GetClip {
    fn from(shortcode: ShortCode) -> Self {
        Self {
            shortcode,
            password: field::Password::default(),
        }
    }
}

impl From<&str> for GetClip {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewClip {
    pub title: field::Title,
    pub content: field::Content,
    pub password: field::Password,
    pub expires_at: field::ExpiresAt,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateClip {
    pub title: field::Title,
    pub content: field::Content,
    pub password: field::Password,
    pub expires_at: field::ExpiresAt,
    pub shortcode: field::ShortCode,
}
