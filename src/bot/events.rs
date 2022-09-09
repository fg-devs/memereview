mod message;

use crate::prelude::{Data, Res};
use poise::{serenity_prelude as serenity, Event};

pub struct Handler;

impl Handler {
    pub async fn listener<'a>(
        ctx: &'a serenity::Context,
        event: &'a Event<'a>,
        data: &'a Data,
    ) -> Res<()> {
        match event {
            Event::Message { new_message } => Self::on_message(ctx, data, new_message).await,
            _ => Ok(()),
        }
    }
}
