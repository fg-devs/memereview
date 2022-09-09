use crate::prelude::Res;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use tracing::{debug, info};

pub mod links;
pub mod users;

pub struct Db {
    pub conn: DatabaseConnection,
}

impl Db {
    pub async fn new() -> Res<Self> {
        debug!("Connecting to Database");
        let conn = Database::connect(env::var("DATABASE_URL")?).await?;
        info!("Connected to Database: {:?}", conn);

        Ok(Self { conn })
    }

    pub async fn run_migrations(&self) -> Res<()> {
        debug!("Running Migrations");
        Migrator::up(&self.conn, None).await?;
        info!("Finished Running Migrations");

        Ok(())
    }
}
