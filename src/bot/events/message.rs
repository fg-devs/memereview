use crate::bot::events::Handler;
use crate::entity;
use crate::entity::links::Entity as Links;
use crate::prelude::{Data, Res};
use poise::serenity_prelude::{Context, Message};
use sea_orm::prelude::*;

impl Handler {
    pub async fn on_message<'a>(ctx: &'a Context, data: &'a Data, msg: &Message) -> Res<()> {
        let db = data.db.as_ref();

        let ldb: Option<entity::links::Model> = Links::find()
            .filter(entity::links::Column::SubmissionsChannelId.eq(12))
            .one(&db.conn)
            .await?;

        if ldb.is_none() {
            return Ok(());
        }

        Ok(())
    }
}
