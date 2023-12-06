use crate::{
    data::{
        model::{Clip, GetClip},
        DataError, DatabasePool,
    },
    domain::clip::field::ShortCode,
};
use sqlx::Row;

use super::model::{self, NewClip};

type Result<T> = std::result::Result<T, DataError>;

pub async fn get_clip<M: Into<model::GetClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model = model.into();
    let shortcode = model.shortcode.as_str();
    Ok(sqlx::query_as!(
        model::Clip,
        r#"SELECT * FROM clips
        WHERE shortcode = ?"#,
        shortcode
    )
    .fetch_one(pool)
    .await?)
}

pub async fn new_clip<M: Into<model::NewClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model: NewClip = model.into();
    let _ = sqlx::query!(
        r#"INSERT INTO clips (
            clip_id,
            shortcode,
            content,
            title,
            posted,
            expires,
            password,
            hits)
            Values (?, ?, ?, ?, ?, ?, ?, ?)"#,
        model.clip_id,
        model.shortcode,
        model.content,
        model.title,
        model.posted,
        model.expires,
        model.password,
        0
    )
    .execute(pool)
    .await?;
    get_clip(model.shortcode, pool).await
}

pub async fn update_clip<M: Into<model::UpdateClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model: model::UpdateClip = model.into();
    let _ = sqlx::query_as!(
        model::Clip,
        r#"UPDATE clips
            SET
            content = ?,
            title = ?,
            expires = ?,
            password = ?
            WHERE
            shortcode = ?;"#,
        model.content,
        model.title,
        model.expires,
        model.password,
        model.shortcode,
    )
    .execute(pool)
    .await?;
    get_clip(model.shortcode, pool).await
}

pub async fn increase_hit_count<M: Into<model::IncreaseHitCount>>(
    model: M,
    pool: &DatabasePool,
) -> Result<()> {
    let model = model.into();
    Ok(sqlx::query!(
        "UPDATE clips SET hits = hits + ? WHERE shortcode = ?",
        model.hits,
        model.shortcode
    )
    .execute(pool)
    .await
    .map(|_| ())?)
}
