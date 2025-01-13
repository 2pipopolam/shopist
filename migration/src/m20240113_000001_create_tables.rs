use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Shops::Table)
                    .col(
                        ColumnDef::new(Shops::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Shops::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Items::Table)
                    .col(
                        ColumnDef::new(Items::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Items::Name).string().not_null())
                    .col(ColumnDef::new(Items::Quantity).integer().not_null())
                    .col(ColumnDef::new(Items::IsPicked).boolean().not_null().default(false))
                    .col(ColumnDef::new(Items::ShopId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-items-shop_id")
                            .from(Items::Table, Items::ShopId)
                            .to(Shops::Table, Shops::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Items::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Shops::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Shops {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum Items {
    Table,
    Id,
    Name,
    Quantity,
    IsPicked,
    ShopId,
}
