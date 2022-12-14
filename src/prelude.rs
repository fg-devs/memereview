use crate::db::Db;
use std::sync::Arc;

pub struct Data {
    pub db: Arc<Db>,
}

pub type Error = color_eyre::eyre::Error;

/// Calling this Res is a temporary workaround until poise fixes the fact that it's macros rely on Result being std::result::Result...
pub type Res<T> = color_eyre::eyre::Result<T>;

pub type Ctx<'a> = poise::Context<'a, Data, Error>;
