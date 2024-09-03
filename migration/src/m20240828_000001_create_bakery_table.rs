use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240828_000001_create_bakery_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Bakery table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Bakery::Table)
                    .if_not_exists()
                    .col(pk_auto(Bakery::Id))
                    .col(string(Bakery::Name).not_null())
                    .col(double(Bakery::ProfitMargin).not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Bakery table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Bakery::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Bakery {
    Table,
    Id,
    Name,
    ProfitMargin,
}
