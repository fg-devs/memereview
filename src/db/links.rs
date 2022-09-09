use super::Db;
use crate::entity;
use crate::entity::links;
use crate::entity::sea_orm_active_enums::RestrictionType;
use crate::prelude::Res;
use poise::serenity_prelude::{ChannelId, UserId};
use sea_orm::prelude::*;
use sea_orm::Set;

impl Db {
    pub async fn add_link(
        &self,
        created_by: UserId,
        submission_channel_id: ChannelId,
        review_channel_id: ChannelId,
        restriction_type: RestrictionType,
    ) -> Res<links::Model> {
        let link = entity::links::ActiveModel {
            created_by: Set(created_by.0 as i64),
            submissions_channel_id: Set(submission_channel_id.0 as i64),
            review_channel_id: Set(review_channel_id.0 as i64),
            restriction_type: Set(restriction_type),
            ..Default::default()
        };

        let res = link.insert(&self.conn).await?;
        Ok(res)
    }
}
