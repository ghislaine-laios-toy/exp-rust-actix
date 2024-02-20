use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Iterable;

use crate::extension::postgres::Type;
use crate::sea_orm::EnumIter;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Text).string().not_null())
                    .col(ColumnDef::new(Post::AuthorId).big_integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("IDX_author_id")
                    .table(Post::Table)
                    .col(Post::AuthorId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Gender)
                    .values(GenderVariant::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Author::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Author::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Author::Name).string().not_null())
                    .col(ColumnDef::new(Author::Gender).custom(Gender).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_author")
                    .from(Post::Table, Post::AuthorId)
                    .to(Author::Table, Author::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Author::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(Gender).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
    AuthorId,
}

#[derive(DeriveIden)]
enum Author {
    Table,
    Id,
    Name,
    Gender,
}

#[derive(DeriveIden)]
struct Gender;

#[derive(DeriveIden, EnumIter)]
enum GenderVariant {
    Male,
    Female,
    Unknown,
}
