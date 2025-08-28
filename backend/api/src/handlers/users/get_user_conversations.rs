use crate::{middlewares::jwt_middleware::JwtAuthData, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::conversation::ConversationsOverview;
use serde_json::json;

#[utoipa::path(
    get,
    operation_id = "Get user conversations",
    tag = "User",
    path = "/api/users/conversations",
    responses(
        (status = 200, description = "Found the conversations and some of their metadata", body=ConversationsOverview),
    )
)]
pub async fn exec(arc: web::Data<Arcadia>, user: JwtAuthData) -> Result<HttpResponse> {
    let conversations = arc.pool.find_user_conversations(user.sub).await?;

    Ok(HttpResponse::Ok().json(json!({"conversations": conversations})))
}
