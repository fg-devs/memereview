use crate::prelude::Res;
use sea_orm::{Database, DatabaseConnection};
use std::env;
use tracing::{debug, info};

pub async fn connect() -> Res<DatabaseConnection> {
    debug!("Connecting to Database");
    let db = Database::connect(env::var("DATABASE_URL").expect("Env DATABASE_URL missing")).await?;
    info!("Connected to Database: {:?}", db);
    Ok(db)
}
