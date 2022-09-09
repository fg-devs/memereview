use crate::entity;
use crate::entity::links::Entity as Links;
use crate::entity::sea_orm_active_enums::RestrictionType;
use crate::extensions::ctx::CtxExt;
use crate::prelude::{Ctx, Error};
use futures::{Stream, StreamExt};
use poise::serenity_prelude::Channel;
use sea_orm::prelude::*;
use std::str::FromStr;

async fn autocomplete_restriction<'a>(
    _ctx: Ctx<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(RestrictionType::get_members())
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name)
}

#[poise::command(
    slash_command,
    subcommands("create", "edit"),
    required_permissions = "MANAGE_CHANNELS"
)]
pub async fn link(_ctx: Ctx<'_>) -> Result<(), Error> {
    panic!("WHAT HOW?????? HUH");
}

#[poise::command(slash_command, guild_only, required_permissions = "MANAGE_CHANNELS")]
pub async fn create(
    ctx: Ctx<'_>,
    #[description = "Submission channel"] submission_channel: Channel,
    #[description = "Review channel"] review_channel: Channel,
    #[description = "Restriction for submissions"]
    #[autocomplete = "autocomplete_restriction"]
    restriction: String,
) -> Result<(), Error> {
    let db = ctx.data().db.as_ref();
    db.add_user(ctx.author().id.0 as i64).await?;

    if Links::find()
        .filter(entity::links::Column::SubmissionsChannelId.eq(submission_channel.id().0))
        .one(&db.conn)
        .await?
        .is_some()
    {
        ctx.say(format!("{} is already linked with {}", submission_channel, review_channel))
            .await?;
        return Ok(());
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

    let link = db
        .add_link(ctx.author().id, submission_channel.id(), review_channel.id(), restriction)
        .await;

    let msg = if link.is_ok() {
        format!("Created with Id: {}", link.unwrap().id)
    } else {
        format!("Something went wrong: {}", submission_channel)
    };

    reply.edit(ctx, |m| m.content(msg).components(|b| b)).await?;

    Ok(())
}

#[poise::command(slash_command, guild_only, required_permissions = "MANAGE_CHANNELS")]
pub async fn edit(
    ctx: Ctx<'_>,
    #[description = "Submission channel"] _submission_channel: Channel,
) -> Result<(), Error> {
    ctx.send(|m| m.content("/link edit isn't implemented properly yet").ephemeral(true)).await?;
    Ok(())
}
