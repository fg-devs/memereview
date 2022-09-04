use crate::prelude::Res;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use tracing::{debug, info};

pub async fn setup() -> Res<DatabaseConnection> {
    debug!("Connecting to Database");
    let db = Database::connect(env::var("DATABASE_URL").expect("Env DATABASE_URL missing")).await?;
    info!("Connected to Database: {:?}", db);

    debug!("Running Migrations");
    Migrator::up(&db, None).await?;
    info!("Finished Running Migrations");

    Ok(db)
}
