use crate::entity;
use crate::prelude::Res;
use sea_orm::ActiveModelTrait;
use sea_orm::Set;
use tracing::info;

use super::Db;

impl Db {
    pub async fn add_user(&self, user_id: i64) -> Res<()> {
        let usr = entity::users::ActiveModel { id: Set(user_id), ..Default::default() };
        if usr.insert(&self.conn).await.is_ok() {
            info!("Creating user with Id: {}", user_id)
        }

        Ok(())
    }
}
