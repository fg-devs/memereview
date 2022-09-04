use crate::m20220904_180014_restriction::Restriction;
use crate::m20220904_180657_users::Users;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Links::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Links::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Links::CreatedBy).big_unsigned().not_null().unique_key())
                    .col(
                        ColumnDef::new(Links::SubmissionsChannelId)
                            .big_unsigned()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Links::ReviewChannelId).big_unsigned().not_null())
                    .col(
                        ColumnDef::new(Links::RestrictionType).custom(Restriction::Type).not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_created_userid")
                    .from(Links::Table, Links::CreatedBy)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Links::Table).to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("FK_created_userid").to_owned()).await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub(crate) enum Links {
    Table,
    Id,
    CreatedBy,
    SubmissionsChannelId,
    ReviewChannelId,
    RestrictionType,
}
