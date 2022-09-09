mod commands;
pub mod embeds;
mod events;

use crate::bot::events::Handler;
use crate::db::Db;
use crate::prelude::{Data, Res};
use commands::*;
use poise::serenity_prelude as serenity;
use std::env;
use std::sync::Arc;

pub async fn start(db: Db) -> Res<()> {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![register(), ping(), link()],
            listener: |ctx, event, _framework, data| {
                Box::pin(async move {
                    Handler::listener(ctx, event, data).await;
                    Ok(())
                })
            },
            ..Default::default()
        })
        .token(env::var("DISCORD_TOKEN").expect("Env DISCORD_TOKEN missing"))
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| {
            Box::pin(async move { Ok(Data { db: Arc::new(db) }) })
        });

    framework.run_autosharded().await?;

    Ok(())
}
