use crate::prelude::{Ctx, Error};
use entity::files;
use entity::sea_orm_active_enums::FileType;
use sea_orm::ActiveModelTrait;
use sea_orm::Set;

#[poise::command(slash_command, prefix_command)]
pub async fn link(ctx: Ctx<'_>) -> Result<(), Error> {
    let db = ctx.data().db.as_ref();
    let file = files::ActiveModel {
        cdn_uri: Set("https://google.com/".to_owned()),
        r#type: Set(FileType::Image),
        ..Default::default()
    };

    file.insert(db).await?;

    ctx.say("linked").await?;
    Ok(())
}
