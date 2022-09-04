use dotenvy::dotenv;
use memereview::logger;
use memereview::prelude::Res;
use memereview::{bot, database};

#[tokio::main]
async fn main() -> Res<()> {
    logger::setup()?;

    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default().into_hooks();

    eyre_hook.install()?;

    std::panic::set_hook(Box::new(move |pi| {
        tracing::error!("{}", panic_hook.panic_report(pi));
    }));

    #[cfg(debug_assertions)]
    dotenv().ok();

    let db = database::setup().await?;

    bot::start(db).await?;

    Ok(())
}
