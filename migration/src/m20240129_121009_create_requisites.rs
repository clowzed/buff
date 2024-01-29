use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Requisites::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Requisites::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Requisites::Name).string().not_null())
                    .col(ColumnDef::new(Requisites::Data).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Requisites::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Requisites {
    Table,
    Id,
    Name,
    Data,
}
