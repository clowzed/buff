use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};

use crate::m20240207_212222_create_chat::Chat;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Sender::Enum)
                    .values(Sender::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Message::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Message::ChatId).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_message_chat")
                            .from(Message::Table, Message::ChatId)
                            .to(Chat::Table, Chat::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Message::Text).string().not_null())
                    .col(
                        ColumnDef::new(Message::Sender)
                            .enumeration(Sender::Enum, vec![Sender::User, Sender::Moderator])
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Message::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Message {
    Table,
    Id,
    ChatId,
    Text,
    Sender,
    CreatedAt,
}

#[derive(Iden, EnumIter)]
enum Sender {
    #[iden = "sender"]
    Enum,
    #[iden = "moderator"]
    Moderator,
    #[iden = "user"]
    User,
}
