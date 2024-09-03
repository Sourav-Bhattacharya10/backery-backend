use sea_orm_migration::{prelude::*, schema::*};
use crate::m20240828_000001_create_bakery_table::Bakery;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240828_000002_create_chef_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Chef table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chef::Table)
                    .if_not_exists()
                    .col(pk_auto(Chef::Id))
                    .col(string(Chef::Name).not_null())
                    .col(json(Chef::ContactDetails))
                    .col(integer(Chef::BakeryId).not_null())
                    .foreign_key(ForeignKey::create()
                                    .name("fk-chef-bakery_id")
                                    .from(Chef::Table, Chef::BakeryId)
                                    .to(Bakery::Table, Bakery::Id)
                                    .on_delete(ForeignKeyAction::Cascade)
                                    .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Chef table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chef::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Chef {
    Table,
    Id,
    Name,
    ContactDetails,
    BakeryId,
}
