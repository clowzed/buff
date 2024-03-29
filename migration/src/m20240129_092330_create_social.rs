use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Social::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Social::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Social::Name).string().not_null())
                    .col(ColumnDef::new(Social::Url).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Social::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Social {
    Table,
    Id,
    Name,
    Url,
}
