use crate::entity;
use crate::entity::sea_orm_active_enums::RestrictionType;
use crate::extensions::ctx::CtxExt;
use crate::prelude::{Ctx, Error};
use futures::{Stream, StreamExt};
use poise::serenity_prelude::Channel;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use std::str::FromStr;
use tracing::info;

async fn autocomplete_restriction<'a>(
    _ctx: Ctx<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(RestrictionType::get_members())
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name)
}

#[poise::command(slash_command, guild_only)]
pub async fn link(
    ctx: Ctx<'_>,
    #[description = "Submission channel"] submission_channel: Channel,
    #[description = "Review channel"] review_channel: Channel,
    #[description = "Restriction for submissions"]
    #[autocomplete = "autocomplete_restriction"]
    restriction: String,
) -> Result<(), Error> {
    let db = ctx.data().db.as_ref();
    let usr =
        entity::users::ActiveModel { id: Set(ctx.author().id.0 as i64), ..Default::default() };

    if usr.insert(db).await.is_ok() {
        info!("Creating user with Id: {}", ctx.author().id)
    }

    let restriction_res = RestrictionType::from_str(restriction.as_str());

    if let Err(e) = restriction_res {
        ctx.send(|m| m.content(format!("{:?}", e)).ephemeral(true)).await?;
        return Ok(());
    }

    let restriction = restriction_res.unwrap();
    let reply = ctx.send_link_result(&submission_channel, &review_channel, &restriction).await?;

    let interaction = reply
        .message()
        .await?
        .await_component_interaction(ctx.discord())
        .author_id(ctx.author().id)
        .await;

    let pressed_button_id = match &interaction {
        Some(m) => &m.data.custom_id,
        None => {
            reply.edit(ctx, |m| m.content("Timed out").components(|b| b)).await?;
            return Ok(());
        }
    };

    if pressed_button_id.eq("link.deny") {
        reply.edit(ctx, |m| m.content("Cancelled").components(|b| b)).await?;
        return Ok(());
    }

    let link = entity::links::ActiveModel {
        created_by: Set(ctx.author().id.0 as i64),
        submissions_channel_id: Set(submission_channel.id().0 as i64),
        review_channel_id: Set(review_channel.id().0 as i64),
        restriction_type: Set(restriction),
        ..Default::default()
    };

    if let Ok(res) = link.clone().insert(db).await {
        reply
            .edit(ctx, |m| m.content(format!("Created with Id: {}", res.id)).components(|b| b))
            .await?;
    } else {
        reply
            .edit(ctx, |m| {
                m.content(format!("Link already exists for channel: {}", submission_channel))
                    .components(|b| b)
            })
            .await?;
    }

    Ok(())
}
