use crate::{handlers::UserId, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_storage::models::conversation::ConversationsOverview;
use serde::Deserialize;
use serde_json::json;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct Params {
    pub user_id: i64,
}

#[utoipa::path(
    get,
    path = "/api/users/{user_id}/conversations",
    responses(
        (status = 200, description = "Found the conversations and some of their metadata", body=ConversationsOverview),
    )
)]
pub async fn exec(
    arc: web::Data<Arcadia>,
    params: web::Path<Params>,
    current_user_id: UserId,
) -> Result<HttpResponse> {
    if !params.user_id.eq(&*current_user_id) {
        return Err(Error::InsufficientPrivileges);
    }
    let conversations = arc.pool.find_user_conversations(current_user_id.0).await?;

    Ok(HttpResponse::Ok().json(json!({"conversations": conversations})))
}
