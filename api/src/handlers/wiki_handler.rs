use crate::{handlers::User, Arcadia};
use actix_web::{HttpResponse, web};
use arcadia_storage::{
    models::wiki::{UserCreatedWikiArticle, WikiArticle},
    repositories::wiki_repository::{create_wiki_article, find_wiki_article},
};
use serde::Deserialize;
use utoipa::IntoParams;
use arcadia_common::error::{Error, Result};

#[utoipa::path(
    post,
    path = "/api/wiki/article",
    responses(
        (status = 200, description = "Successfully created the wiki article", body=WikiArticle),
    )
)]
pub async fn add_wiki_article(
    article: web::Json<UserCreatedWikiArticle>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    if current_user.class != "staff" {
        return Err(Error::InsufficientPrivileges);
    }

    let article = create_wiki_article(arc.pool.borrow(), &article, current_user.id).await?;

    Ok(HttpResponse::Created().json(article))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetWikiArticleQuery {
    id: i64,
}

#[utoipa::path(
    get,
    path = "/api/wiki/article",
    params(GetWikiArticleQuery),
    responses(
        (status = 200, description = "Successfully found the wiki article", body=WikiArticle),
    )
)]
pub async fn get_wiki_article(
    query: web::Query<GetWikiArticleQuery>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    let article = find_wiki_article(arc.pool.borrow(), query.id).await?;

    Ok(HttpResponse::Ok().json(article))
}
