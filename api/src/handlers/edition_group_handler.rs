use crate::{
    Arcadia, handlers::UserId,
};
use actix_web::{HttpResponse, web};
use arcadia_common::error::Result;
use arcadia_storage::{
  models::edition_group::{EditionGroup, UserCreatedEditionGroup},
  repositories::edition_group_repository::create_edition_group,
};

#[utoipa::path(
    post,
    path = "/api/edition-group",
    responses(
        (status = 200, description = "Successfully created the edition_group", body=EditionGroup),
    )
)]
pub async fn add_edition_group(
    form: web::Json<UserCreatedEditionGroup>,
    arc: web::Data<Arcadia>,
    current_user_id: UserId,
) -> Result<HttpResponse> {
    let edition_group = create_edition_group(arc.pool.borrow(), &form, current_user_id.0).await?;

    Ok(HttpResponse::Created().json(edition_group))
}
