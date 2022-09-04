mod commands;

use crate::prelude::{Ctx, Data, Error, Res};
use commands::*;
use poise::serenity_prelude as serenity;
use sea_orm::DatabaseConnection;
use std::env;
use std::sync::Arc;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Ctx<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

pub async fn start(db: DatabaseConnection) -> Res<()> {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), register(), ping(), link()],
            ..Default::default()
        })
        .token(env::var("DISCORD_TOKEN").expect("Env DISCORD_TOKEN missing"))
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| {
            Box::pin(async move { Ok(Data { db: Arc::new(db) }) })
        });

    framework.run().await?;

    Ok(())
}
