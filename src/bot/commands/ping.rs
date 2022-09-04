use crate::prelude::{Ctx, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Ctx<'_>) -> Result<(), Error> {
    ctx.say("pong").await?;
    Ok(())
}
