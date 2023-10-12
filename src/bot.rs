mod commands;
pub mod embeds;
mod events;

use crate::bot::events::Handler;
use crate::db::Db;
use crate::prelude::{Data, Res};
use commands::*;
use poise::serenity_prelude::GatewayIntents;
use std::env;
use std::sync::Arc;

pub async fn start(db: Db) -> Res<()> {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![register(), ping(), link()],
            event_handler: |_ctx, _event, _framework, _data| {
                Box::pin(async move {
                    Handler::listener(_ctx, _event, _framework.user_data).await?;
                    Ok(())
                })
            },
            ..Default::default()
        })
        .token(env::var("DISCORD_TOKEN")?)
        .intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move { Ok(Data { db: Arc::new(db) }) })
        });

    framework.run_autosharded().await?;

    Ok(())
}
