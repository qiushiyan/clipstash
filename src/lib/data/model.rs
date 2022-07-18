use crate::data::DatabaseId;
use crate::domain::clip;
use crate::{ClipError, ShortCode, Time};
use chrono::NaiveDateTime;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub id: String,
    pub title: Option<String>,
    pub content: String,
    pub shortcode: String,
    pub created_at: NaiveDateTime,
    pub expires_at: Option<NaiveDateTime>,
    pub password: Option<String>,
    pub hits: u64,
}

impl TryFrom<Clip> for crate::domain::Clip {
    type Error = ClipError;

    fn try_from(clip: Clip) -> Result<Self, Self::Error> {
        use crate::domain::clip::field;

        Ok(Self {
            id: field::ClipId::new(DatabaseId::from_str(clip.id.as_str())?),
            title: field::Title::new(clip.title),
            content: field::Content::new(clip.content.as_str())?,
            shortcode: field::ShortCode::from(clip.shortcode),
            created_at: field::CreatedAt::new(Time::from_naive_utc(clip.created_at)),
            expires_at: field::ExpiresAt::new(clip.expires_at.map(|dt| Time::from_naive_utc(dt))),
            password: field::Password::new(clip.password.unwrap_or_default())?,
            hits: field::Hits::new(clip.hits),
        })
    }
}
