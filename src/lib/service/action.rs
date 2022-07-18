use crate::data::{query, DatabasePool, Transaction};
use crate::service::ask;
use crate::{Clip, ShortCode};
use std::convert::TryInto;

use super::ServiceError;

pub async fn get_clip(req: ask::GetClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    let password = req.password.clone();
    let clip: Clip = query::get_clip(req, pool).await?.try_into()?;
    if clip.password.has_password() {
        if clip.password == password {
            Ok(clip)
        } else {
            Err(ServiceError::PermissionError("invalid password".to_owned()))
        }
    } else {
        Ok(clip)
    }
}

pub async fn new_clip(req: ask::NewClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    let clip: Clip = query::new_clip(req, pool).await?.try_into()?;
    Ok(clip)
}

pub async fn update_clip(req: ask::UpdateClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    let clip: Clip = query::update_clip(req, pool).await?.try_into()?;
    Ok(clip)
}
