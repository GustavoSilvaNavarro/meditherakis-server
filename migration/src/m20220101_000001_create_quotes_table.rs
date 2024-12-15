use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Quotes::Table)
                    .if_not_exists()
                    .col(pk_auto(Quotes::Id))
                    .col(ColumnDef::new(Quotes::Quote).text().not_null().unique_key())
                    .col(
                        ColumnDef::new(Quotes::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default("now()"),
                    )
                    .col(
                        ColumnDef::new(Quotes::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default("now()"),
                    )
                    .col(
                        ColumnDef::new(Quotes::DeletedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-quotes_quote")
                    .table(Quotes::Table)
                    .col(Quotes::Quote)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx-quotes_quote").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Quotes::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Quotes {
    Table,
    Id,
    Quote,
    #[sea_orm(column_name = "created_at")]
    CreatedAt,
    #[sea_orm(column_name = "updated_at")]
    UpdatedAt,
    #[sea_orm(column_name = "deleted_at")]
    DeletedAt,
}
