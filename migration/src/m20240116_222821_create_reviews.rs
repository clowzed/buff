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
                    .table(Review::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Review::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Review::SteamId).big_unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_reviews_user")
                            .from(Review::Table, Review::SteamId)
                            .to(User::Table, User::SteamId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Review::Review).string().not_null())
                    .col(ColumnDef::new(Review::Stars).not_null().tiny_integer())
                    .col(
                        ColumnDef::new(Review::CreatedAt)
                            .not_null()
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    // Between is inclusive
                    .check(Expr::col(Review::Stars).between(0, 5))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Review::Table).to_owned())
            .await
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(DeriveIden)]
enum Review {
    Table,
    Id,
    Review,
    CreatedAt,
    Stars,
    SteamId,
}
