use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Restriction::Type)
                    .values(vec![
                        Restriction::None,
                        Restriction::Image,
                        Restriction::Video,
                        Restriction::Attachment,
                        Restriction::Text,
                    ])
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_type(Type::drop().name(Restriction::Type).to_owned()).await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
pub(crate) enum Restriction {
    Type,
    None,
    Image,
    Video,
    Attachment,
    Text,
}

impl Iden for Restriction {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Type => "restriction_type",
                Self::None => "none",
                Self::Image => "image",
                Self::Video => "video",
                Self::Attachment => "attachment",
                Self::Text => "text",
            }
        )
        .unwrap()
    }
}
