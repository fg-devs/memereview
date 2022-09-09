pub use sea_orm_migration::prelude::*;

mod m20220831_222057_files;
mod m20220904_180014_restriction;
mod m20220904_180657_users;
mod m20220904_181126_links;
mod m20220909_133340_submissions;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220831_222057_files::Migration),
            Box::new(m20220904_180014_restriction::Migration),
            Box::new(m20220904_180657_users::Migration),
            Box::new(m20220904_181126_links::Migration),
            Box::new(m20220909_133340_submissions::Migration),
        ]
    }
}
