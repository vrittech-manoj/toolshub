use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tools::Table)
                    .if_not_exists()
                    .col(pk_auto(Tools::Id))
                    .col(string(Tools::Title))
                    .col(string(Tools::Url))
                    .col(string(Tools::Ispublished))
                    .col(string(Tools::Ispremium))
                    .col(string(Tools::Rating))
                    .col(string(Tools::Secure))
                    .col(string(Tools::Image))
                    .col(string(Tools::Decription))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tools::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Tools {
    Table,
    Id,
    Title,
    Url,
    Ispublished,
    Ispremium,
    Rating,
    Secure,
    Image,
    Decription,
}