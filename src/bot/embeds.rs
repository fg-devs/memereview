use chrono::Utc;
use poise::serenity_prelude::CreateEmbed;

pub fn create_basic_embed() -> CreateEmbed {
    let mut e = CreateEmbed::default();

    e.timestamp(Utc::now());
    e
}
