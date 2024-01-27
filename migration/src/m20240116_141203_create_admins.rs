use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Role::Enum)
                    .values(Role::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Admin::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Admin::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Admin::Login).string().not_null())
                    .col(ColumnDef::new(Admin::Password).string().not_null())
                    .col(
                        ColumnDef::new(Admin::Role)
                            .enumeration(Role::Enum, vec![Role::Admin, Role::Moderator])
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Admin::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Admin {
    Table,
    Id,
    Login,
    Password,
    Role,
}

#[derive(Iden, EnumIter)]
enum Role {
    #[iden = "role"]
    Enum,
    #[iden = "moderator"]
    Moderator,
    #[iden = "admin"]
    Admin,
}
