use sea_orm_migration::prelude::*;

use crate::m20240116_115527_create_users::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Blacklisted::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Blacklisted::Id)
                            .big_integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Blacklisted::SteamId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_blacklisted_user")
                            .from(Blacklisted::Table, Blacklisted::SteamId)
                            .to(User::Table, User::SteamId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Blacklisted::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Blacklisted {
    Table,
    //? We need this because
    //*    Entity crashes on derive without primary key
    Id,
    SteamId,
}
