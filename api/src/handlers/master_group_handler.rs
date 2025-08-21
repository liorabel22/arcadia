use crate::{
    Arcadia, handlers::UserId,
};
use actix_web::{HttpResponse, web};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::master_group::{MasterGroup, UserCreatedMasterGroup},
    repositories::master_group_repository::create_master_group,
};

#[utoipa::path(
    post,
    path = "/api/master-group",
    responses(
        (status = 200, description = "Successfully created the master group", body=MasterGroup),
    )
)]
pub async fn add_master_group(
    form: web::Json<UserCreatedMasterGroup>,
    arc: web::Data<Arcadia>,
    current_user_id: UserId,
) -> Result<HttpResponse> {
    let master_group = create_master_group(arc.pool.borrow(), &form, current_user_id.0).await?;

    Ok(HttpResponse::Created().json(master_group))
}
