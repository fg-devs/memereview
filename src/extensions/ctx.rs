use crate::bot::embeds::create_basic_embed;
use crate::entity::sea_orm_active_enums::RestrictionType;
use crate::prelude::Ctx;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{Channel, CreateEmbed};
use poise::{async_trait, ReplyHandle};

#[async_trait]
pub trait CtxExt {
    async fn send_embed<F>(&self, build: F) -> Result<ReplyHandle, serenity::Error>
    where
        F: FnOnce(&mut CreateEmbed) + Send + Sync;
    async fn send_link_result(
        &self,
        submission_channel: &Channel,
        review_channel: &Channel,
        restriction: &RestrictionType,
    ) -> Result<ReplyHandle, serenity::Error>;
}

#[async_trait]
impl CtxExt for Ctx<'_> {
    async fn send_embed<F>(&self, build: F) -> Result<ReplyHandle, serenity::Error>
    where
        F: FnOnce(&mut CreateEmbed) + Send + Sync,
    {
        let mut e = create_basic_embed();
        build(&mut e);

        self.send(|m| {
            m.allowed_mentions(|f| f.replied_user(false));
            m.embeds.push(e);
            m
        })
        .await
    }

    async fn send_link_result(
        &self,
        submission_channel: &Channel,
        review_channel: &Channel,
        restriction: &RestrictionType,
    ) -> Result<ReplyHandle, serenity::Error> {
        let mut description = String::new();
        description += format!("> Submissions: {}\n", submission_channel).as_str();
        description += format!("> Review: {}\n", review_channel).as_str();
        description += format!("> Restriction: {} {}\n", restriction.emoji(), restriction).as_str();
        description += "\nAre these correct?";

        self.send(|m| {
            m.allowed_mentions(|f| f.replied_user(false));
            m.content(description);
            m.components(|c| {
                c.create_action_row(|r| {
                    r.create_button(|b| {
                        b.custom_id("link.confirm")
                            .label("Confirm")
                            .style(serenity::ButtonStyle::Success)
                    })
                    .create_button(|b| {
                        b.custom_id("link.deny")
                            .label("Cancel")
                            .style(serenity::ButtonStyle::Danger)
                    })
                })
            });
            m
        })
        .await
    }
}
