use sea_orm_migration::prelude::*;

use crate::m20240207_221530_create_messages::Message;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Image::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Image::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Image::Path).string().not_null())
                    .col(ColumnDef::new(Image::MessageId).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_image_message")
                            .from(Image::Table, Image::MessageId)
                            .to(Message::Table, Message::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Image::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Image {
    Table,
    Id,
    Path,
    MessageId,
}
