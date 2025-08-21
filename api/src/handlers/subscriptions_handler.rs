use crate::{handlers::User, Arcadia};
use actix_web::{HttpResponse, web};
use arcadia_storage::repositories::subscriptions_repository::{create_subscription, delete_subscription};
use serde::Deserialize;
use utoipa::IntoParams;
use arcadia_common::error::Result;

#[derive(Debug, Deserialize, IntoParams)]
pub struct AddSubscriptionQuery {
    item_id: i64,
    item: String,
}

#[utoipa::path(
    post,
    path = "/api/subscription",
    params (RemoveSubscriptionQuery),
    responses(
        (status = 200, description = "Successfully subscribed to the item"),
    )
)]
pub async fn add_subscription(
    query: web::Query<AddSubscriptionQuery>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    create_subscription(arc.pool.borrow(), query.item_id, &query.item, current_user.id).await?;

    Ok(HttpResponse::Created().json(serde_json::json!({"result": "success"})))
}

pub type RemoveSubscriptionQuery = AddSubscriptionQuery;

#[utoipa::path(
    delete,
    path = "/api/subscription",
    params (RemoveSubscriptionQuery),
    responses(
        (status = 200, description = "Successfully unsubscribed to the item"),
    )
)]
pub async fn remove_subscription(
    query: web::Query<RemoveSubscriptionQuery>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    delete_subscription(arc.pool.borrow(), query.item_id, &query.item, current_user.id).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"result": "success"})))
}
