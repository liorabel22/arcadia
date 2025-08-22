use crate::{Arcadia, handlers::User};
use actix_web::{HttpResponse, web};
use arcadia_common::error::Result;
use arcadia_storage::models::user::EditedUser;
use serde_json::json;

#[utoipa::path(
    put,
    path = "/api/user",
    responses(
        (status = 200, description = "Successfully edited the user"),
    )
)]
pub async fn exec(
    form: web::Json<EditedUser>,
    current_user: User,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    arc.pool.update_user(current_user.id, &form).await?;

    Ok(HttpResponse::Ok().json(json!({"status": "success"})))
}
