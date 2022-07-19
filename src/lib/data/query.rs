use super::model;
use crate::data::{DataError, DatabasePool};
use crate::web::ApiKey;
use crate::ShortCode;
use sqlx::Row;

type Result<T> = std::result::Result<T, DataError>;

pub async fn get_clip<M>(m: M, pool: &DatabasePool) -> Result<model::Clip>
where
    M: Into<model::GetClip>,
{
    let m: model::GetClip = m.into();
    let shortcode = m.shortcode.as_str();
    Ok(sqlx::query_as!(
        model::Clip,
        r#" SELECT * FROM clips WHERE shortcode = ?"#,
        shortcode
    )
    .fetch_one(pool)
    .await?)
}

pub async fn new_clip<M>(m: M, pool: &DatabasePool) -> Result<model::Clip>
where
    M: Into<model::NewClip>,
{
    let m = m.into();
    let _ = sqlx::query!(
        r#"
        INSERT INTO clips
        (id, title, content, password, shortcode, created_at, expires_at, hits)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        m.id,
        m.title,
        m.content,
        m.password,
        m.shortcode,
        m.created_at,
        m.expires_at,
        0_i64
    )
    .execute(pool)
    .await?;
    get_clip(m.shortcode, pool).await
}

pub async fn update_clip<M>(m: M, pool: &DatabasePool) -> Result<model::Clip>
where
    M: Into<model::UpdateClip>,
{
    let m = m.into();
    let _ = sqlx::query!(
        r#"UPDATE clips SET title = ?, content = ?, password = ?, expires_at = ? WHERE shortcode = ?"#,
        m.title,
        m.content,
        m.password,
        m.expires_at,
        m.shortcode
    )
    .execute(pool)
    .await?;
    get_clip(m.shortcode, pool).await
}

pub async fn increment_hit(shortcode: &ShortCode, hits: i64, pool: &DatabasePool) -> Result<()> {
    let shortcode = shortcode.as_str();
    Ok(sqlx::query!(
        r#"UPDATE clips SET hits = hits + ? WHERE shortcode = ?"#,
        hits,
        shortcode
    )
    .execute(pool)
    .await
    .map(|_| ())?)
}

pub async fn save_api_key(api_key: ApiKey, pool: &DatabasePool) -> Result<ApiKey> {
    let bytes = api_key.clone().into_inner();
    let _ = sqlx::query!("INSERT INTO api_keys (api_key) VALUES (?)", bytes)
        .execute(pool)
        .await
        .map(|_| ())?;
    Ok(api_key)
}
/// The return value from the [`revoke_api_key`] function.
pub enum RevocationStatus {
    /// The [`ApiKey`] was successfully revoked.
    Revoked,
    /// The [`ApiKey`] was not found, so no revocation occuured.
    NotFound,
}

/// Revokes an [`ApiKey`].
pub async fn revoke_api_key(api_key: ApiKey, pool: &DatabasePool) -> Result<RevocationStatus> {
    let bytes = api_key.clone().into_inner();
    Ok(
        sqlx::query!("DELETE FROM api_keys WHERE api_key == ?", bytes)
            .execute(pool)
            .await
            .map(|result| match result.rows_affected() {
                0 => RevocationStatus::NotFound,
                _ => RevocationStatus::Revoked,
            })?,
    )
}

/// Determines if the [`ApiKey`] is valid.
pub async fn api_key_is_valid(api_key: ApiKey, pool: &DatabasePool) -> Result<bool> {
    let bytes = api_key.clone().into_inner();
    Ok(
        sqlx::query("SELECT COUNT(api_key) FROM api_keys WHERE api_key = ?")
            .bind(bytes)
            .fetch_one(pool)
            .await
            .map(|row| {
                let count: u32 = row.get(0);
                count > 0
            })?,
    )
}

/// Deletes all expired [`Clips`](`crate::domain::Clip`).
pub async fn delete_expired(pool: &DatabasePool) -> Result<u64> {
    Ok(
        sqlx::query!(r#"DELETE FROM clips WHERE strftime('%s', 'now') > expires_at"#)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub mod test_helpers {
    use crate::data::*;
    pub fn model_get_clip(shortcode: &str) -> model::GetClip {
        model::GetClip {
            shortcode: shortcode.into(),
        }
    }

    pub fn model_new_clip(shortcode: &str) -> model::NewClip {
        use chrono::Utc;
        model::NewClip {
            id: DatabaseId::new().into(),
            title: None,
            content: format!("content for clip '{}'", shortcode),
            shortcode: shortcode.into(),
            created_at: Utc::now().timestamp(),
            expires_at: None,
            password: None,
        }
    }
}
