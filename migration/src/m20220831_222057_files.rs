use crate::extension::postgres::Type;
use sea_orm_migration::prelude::*;

pub struct Migration;

#[derive(Iden)]
enum Files {
    Table,
    Id,
    CdnUri,
    Type,
}

enum FileType {
    Type,
    Video,
    Image,
    Other,
}

impl Iden for FileType {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Type => "file_type",
                Self::Video => "video",
                Self::Image => "image",
                Self::Other => "other",
            }
        )
        .unwrap()
    }
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220831_222057_files"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(FileType::Type)
                    .values(vec![FileType::Video, FileType::Image, FileType::Other])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Files::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Files::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Files::CdnUri).string().not_null())
                    .col(ColumnDef::new(Files::Type).custom(FileType::Type).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        todo!()
    }
}
