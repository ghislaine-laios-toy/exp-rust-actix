use actix_web::{delete, get, post, put, Responder, web};
use anyhow::{anyhow, Context};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryOrder, QuerySelect, TryIntoModel,
};
use sea_orm::ActiveValue::{Set, Unchanged};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::entities::author;
use crate::entities::author::Model;
use crate::entities::prelude::Author;
use crate::entities::sea_orm_active_enums::Gender;
use crate::error::Error;

#[post("")]
async fn create(
    author_info: web::Json<AuthorInfo>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let mutation = Mutation(&app_state.db_coon);

    // TODO: Implement robust error handling.
    let author = mutation
        .create_author(author_info.into_inner())
        .await
        .context("failed to create the provided author")?;
    let author = author
        .try_into_model()
        .context("an error occurred when converting author::ActivatedModel to author::Model")?;

    Ok::<_, Error>(web::Json(author))
}

#[get("")]
async fn list(
    params: web::Query<ListAuthorsParams>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let authors_list = Query(&app_state.db_coon)
        .list_authors(params.into_inner())
        .await
        .context("failed to retrieve authors list")?;

    Ok::<_, Error>(web::Json(authors_list))
}
#[put("/{author_id}")]
async fn put(
    id: web::Path<(u64,)>,
    author_info: web::Json<AuthorInfo>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let id = id.into_inner().0 as i64;
    let author_info = author_info.into_inner();

    let author = Query(&app_state.db_coon)
        .find_by_id(id)
        .await
        .context(format!("failed to find the author with the id {}", id))
        .map_err(Error::InternalError)?;

    let Some(author) = author else {
        return Err(Error::InternalError(anyhow!(
            "the author with id {} is not found",
            id
        )));
    };

    let author = Mutation(&app_state.db_coon)
        .update_author(author.id, author_info)
        .await
        .context(format!("failed to update the author with the id {}", id))
        .map_err(Error::InternalError)?;

    Ok::<_, Error>(web::Json(author))
}

#[delete("/{author_id}")]
async fn delete() -> impl Responder {
    ""
}


#[derive(Debug, Deserialize)]
struct AuthorInfo {
    name: String,
    gender: Gender,
}

#[derive(Debug, Deserialize)]
struct ListAuthorsParams {
    start_id: Option<u64>,
    number: Option<u64>,
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/authors").service(create).service(list).service(put));
}

struct Mutation<'a>(&'a DatabaseConnection);

impl Mutation<'_> {
    async fn create_author(&self, author_info: AuthorInfo) -> Result<author::ActiveModel, DbErr> {
        author::ActiveModel {
            name: Set(author_info.name),
            gender: Set(author_info.gender),
            ..Default::default()
        }
        .save(self.0)
        .await
    }

    async fn update_author(&self, id: i64, author_info: AuthorInfo) -> Result<Model, DbErr> {
        author::ActiveModel {
            id: Unchanged(id),
            name: Set(author_info.name),
            gender: Set(author_info.gender),
        }
        .update(self.0)
        .await
    }
}

struct Query<'a>(&'a DatabaseConnection);

impl Query<'_> {
    async fn list_authors(&self, params: ListAuthorsParams) -> Result<Vec<author::Model>, DbErr> {
        Author::find()
            .order_by_asc(author::Column::Id)
            .limit(params.number)
            .offset(params.start_id)
            .all(self.0)
            .await
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Model>, DbErr> {
        Author::find_by_id(id).one(self.0).await
    }
}
