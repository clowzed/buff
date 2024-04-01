use crate::{
    m20240116_115527_create_users::User, m20240116_141203_create_admins::Admin, sea_orm::EnumIter,
};
use sea_orm_migration::{prelude::*, sea_orm::Iterable, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Status::Enum)
                    .values(Status::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Order::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Order::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Order::PaymentMethod).string().not_null())
                    .col(
                        ColumnDef::new(Order::Status)
                            .enumeration(
                                Status::Enum,
                                [Status::Cancelled, Status::Created, Status::Succeeded],
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(Order::Requisites).text().not_null())
                    .col(
                        ColumnDef::new(Order::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Order::FinishedAt).date_time())
                    .col(ColumnDef::new(Order::SteamId).big_unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_order_user")
                            .from(Order::Table, Order::SteamId)
                            .to(User::Table, User::SteamId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Order::ModeratorId).big_integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_order_admin")
                            .from(Order::Table, Order::ModeratorId)
                            .to(Admin::Table, Admin::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .col(ColumnDef::new(Order::Amount).decimal().not_null())
                    .col(
                        ColumnDef::new(Order::FixedCurrencyRate)
                            .decimal()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Order::CurrencySymbol).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Order::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Order {
    Table,
    Id,
    CreatedAt,
    PaymentMethod,
    Status,
    SteamId,
    ModeratorId,
    Amount,
    FinishedAt,
    FixedCurrencyRate,
    Requisites,
    CurrencySymbol,
}

#[derive(Iden, EnumIter)]
enum Status {
    #[iden = "status"]
    Enum,
    #[iden = "created"]
    Created,
    #[iden = "cancelled"]
    Cancelled,
    #[iden = "succeeded"]
    Succeeded,
}
