use super::model;
use crate::data::{DataError, DatabasePool};
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
        r#"UPDATE clips SET hits = ? + 1 WHERE shortcode = ?"#,
        hits,
        shortcode
    )
    .execute(pool)
    .await
    .map(|_| ())?)
}
