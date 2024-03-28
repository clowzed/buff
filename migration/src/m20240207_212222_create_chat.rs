use sea_orm_migration::prelude::*;

use crate::{
    m20240116_115527_create_users::User, m20240116_141203_create_admins::Admin,
    m20240117_153036_create_orders::Order,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chat::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Chat::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Chat::ModeratorId).big_integer().not_null())
                    .col(ColumnDef::new(Chat::SteamId).big_integer().not_null())
                    .col(ColumnDef::new(Chat::OrderId).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_chat_order")
                            .from(Chat::Table, Chat::OrderId)
                            .to(Order::Table, Order::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_chat_user")
                            .from(Chat::Table, Chat::SteamId)
                            .to(User::Table, User::SteamId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_chat_admin")
                            .from(Chat::Table, Chat::ModeratorId)
                            .to(Admin::Table, Admin::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chat::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Chat {
    Table,
    Id,
    ModeratorId,
    SteamId,
    OrderId,
}
