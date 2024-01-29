use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(VideoReview::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(VideoReview::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(VideoReview::Url).string().not_null())
                    .col(ColumnDef::new(VideoReview::Name).string().not_null())
                    .col(ColumnDef::new(VideoReview::Avatar).string().not_null())
                    .col(ColumnDef::new(VideoReview::Subscribers).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(VideoReview::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum VideoReview {
    Table,
    Id,
    Url,
    Name,
    Avatar,
    Subscribers,
}
