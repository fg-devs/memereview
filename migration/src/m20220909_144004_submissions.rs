use crate::m20220831_222057_files::Files;
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
                    .table(Submissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Submissions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Submissions::AttachmentId).integer().not_null())
                    .col(ColumnDef::new(Submissions::Text).string().not_null())
                    .col(ColumnDef::new(Submissions::SubmittedBy).big_unsigned().not_null())
                    .col(ColumnDef::new(Submissions::Approved).boolean())
                    .col(ColumnDef::new(Submissions::HandledBy).big_unsigned())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_attachment_fileid")
                    .from(Submissions::Table, Submissions::AttachmentId)
                    .to(Files::Table, Files::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_submitted_userid")
                    .from(Submissions::Table, Submissions::SubmittedBy)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_handled_userid")
                    .from(Submissions::Table, Submissions::HandledBy)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Submissions::Table).to_owned()).await?;
        manager
            .drop_foreign_key(ForeignKey::drop().name("FK_attachment_fileid").to_owned())
            .await?;
        manager.drop_foreign_key(ForeignKey::drop().name("FK_submitted_userid").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("FK_handled_userid").to_owned()).await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Submissions {
    Table,
    Id,
    AttachmentId,
    Text,
    SubmittedBy,
    Approved,
    HandledBy,
}
