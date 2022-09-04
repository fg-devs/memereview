use crate::prelude::{Ctx, Error};
use poise::serenity_prelude::CreateSelectMenuOption;

#[poise::command(slash_command, prefix_command)]
pub async fn link(ctx: Ctx<'_>) -> Result<(), Error> {
    let db = ctx.data().db.as_ref();
    let reply = ctx
        .send(|m| {
            m.content("Choose an option").components(|c| {
                c.create_action_row(|r| {
                    r.create_select_menu(|s| {
                        s.custom_id("test").placeholder("No option selected").options(|f| {
                            f.add_option(CreateSelectMenuOption::new("Option 1", "option_1"))
                                .add_option(CreateSelectMenuOption::new("Option 2", "option_2"))
                        })
                    })
                })
            })
        })
        .await?;

    ctx.say("linked").await?;
    Ok(())
}
