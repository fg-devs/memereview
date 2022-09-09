use crate::bot::events::Handler;
use crate::entity;
use crate::entity::links::Entity as Links;
use crate::prelude::{Data, Res};
use crate::services::attachments::Attachments;
use poise::serenity_prelude::{Context, Message};
use sea_orm::prelude::*;
use std::borrow::BorrowMut;

impl Handler {
    pub async fn on_message<'a>(ctx: &'a Context, data: &'a Data, msg: &Message) -> Res<()> {
        let db = data.db.as_ref();

        let ldb: Option<entity::links::Model> = Links::find()
            .filter(entity::links::Column::SubmissionsChannelId.eq(msg.channel_id.0))
            .one(&db.conn)
            .await?;

        if ldb.is_none() {
            return Ok(());
        }

        let mut attachments = Attachments::global().lock().await;
        let mut files: Vec<String> = Vec::default();

        if !msg.attachments.is_empty() {
            for att in msg.attachments.iter() {
                let upload = attachments.upload(att.url.clone()).await;
                if let Ok(url) = upload {
                    files.push(url)
                }
            }
        }

        println!("{:?}", files);

        Ok(())
    }
}
