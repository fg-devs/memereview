use crate::prelude::{Ctx, Error};

#[poise::command(prefix_command)]
pub async fn register(ctx: Ctx<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
