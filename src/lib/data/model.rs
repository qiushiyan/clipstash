use crate::data::DatabaseId;
use crate::domain::clip;
use crate::{ClipError, ShortCode, Time};
use chrono::NaiveDateTime;
use derive_more::From;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub(in crate::data) id: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) content: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) created_at: NaiveDateTime,
    pub(in crate::data) expires_at: Option<NaiveDateTime>,
    pub(in crate::data) password: Option<String>,
    pub(in crate::data) hits: i64,
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

#[derive(From)]
pub struct GetClip {
    pub(in crate::data) shortcode: String,
}

impl From<ShortCode> for GetClip {
    fn from(shortcode: ShortCode) -> Self {
        Self {
            shortcode: shortcode.into_inner(),
        }
    }
}

pub struct NewClip {
    pub(in crate::data) id: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) content: String,
    pub(in crate::data) password: Option<String>,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) created_at: i64,
    pub(in crate::data) expires_at: Option<i64>,
}

pub struct UpdateClip {
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) content: String,
    pub(in crate::data) password: Option<String>,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) expires_at: Option<i64>,
}
