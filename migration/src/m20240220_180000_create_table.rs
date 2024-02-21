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
                    .col(
                        ColumnDef::new(Author::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Author::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(Author::Gender).custom(Gender).not_null())
                    .to_owned(),
            )
            .await?;

        let post_table = Table::create()
            .table(Post::Table)
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
            .foreign_key(
                ForeignKey::create()
                    .name("FK_author")
                    .from(Post::Table, Post::AuthorId)
                    .to(Author::Table, Author::Id),
            )
            .to_owned();
        // println!("{}", post_table.to_string(PostgresQueryBuilder));
        manager.create_table(post_table).await?;

        let author_id_index = Index::create()
            .name("IDX_author_id")
            .table(Post::Table)
            .col(Post::AuthorId)
            .to_owned();
        manager.create_index(author_id_index).await?;

        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .col(
                        ColumnDef::new(Tag::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tag::Name).string().not_null())
                    .col(ColumnDef::new(Tag::Description).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PostTag::Table)
                    .col(ColumnDef::new(PostTag::PostId).big_integer().not_null())
                    .col(ColumnDef::new(PostTag::TagId).big_integer().not_null())
                    .primary_key(Index::create().col(PostTag::PostId).col(PostTag::TagId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_post_id")
                            .from(PostTag::Table, PostTag::PostId)
                            .to(Post::Table, Post::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_tag_id")
                            .from(PostTag::Table, PostTag::TagId)
                            .to(Tag::Table, Tag::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PostTag::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Author::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(Gender).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
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

#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
    Name,
    Description,
}

#[derive(DeriveIden)]
enum PostTag {
    Table,
    PostId,
    TagId,
}
