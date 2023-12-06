use rocket::http::hyper::service::Service;
use sqlx::pool;

use crate::{
    data::{query, DatabasePool, Transaction},
    domain::clip::{field::ShortCode, Clip, ClipError},
    service::{ask, ServiceError},
};

pub async fn get_clip(req: ask::GetClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    let user_password = req.password.clone();
    let clip: Clip = query::get_clip(req, pool).await?.try_into()?;
    if clip.password.has_password() && clip.password != user_password {
        Err(ServiceError::PermissionError("Invalid password".to_owned()))
    } else {
        Ok(clip)
    }
}

pub async fn new_clip(req: ask::NewClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    Ok(query::new_clip(req, pool).await?.try_into()?)
}

pub async fn update_clip(req: ask::UpdateClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    Ok(query::update_clip(req, pool).await?.try_into()?)
}

pub async fn increase_hit_count(
    req: ask::IncreaseHitCount,
    pool: &DatabasePool,
) -> Result<(), ServiceError> {
    Ok(query::increase_hit_count(req, pool).await?)
}

pub async fn begin_transaction(pool: &DatabasePool) -> Result<Transaction<'_>, ServiceError> {
    Ok(pool.begin().await?)
}

pub async fn end_transaction(transaction: Transaction<'_>) -> Result<(), ServiceError> {
    Ok(transaction.commit().await?)
}
