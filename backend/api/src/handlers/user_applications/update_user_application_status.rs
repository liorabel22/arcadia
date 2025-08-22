use crate::{Arcadia, handlers::User};
use actix_web::{HttpResponse, web};
use arcadia_common::error::{Error, Result};
use arcadia_storage::models::user_application::{UserApplication, UserApplicationStatus};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct UpdateUserApplication {
    pub status: UserApplicationStatus,
    pub user_application_id: i64,
}

#[utoipa::path(
    put,
    path = "/api/user-application",
    request_body = UpdateUserApplication,
    responses(
        (status = 200, description = "Successfully updated user application status", body = UserApplication),
        (status = 403, description = "Forbidden - Only staff members can update user applications"),
        (status = 404, description = "User application not found")
    )
)]
pub async fn exec(
    arc: web::Data<Arcadia>,
    user: User,
    form: web::Json<UpdateUserApplication>,
) -> Result<HttpResponse> {
    // Check if user is staff
    if user.class != "staff" {
        return Err(Error::InsufficientPrivileges);
    }

    let updated_application = arc
        .pool
        .update_user_application_status(form.user_application_id, form.status.clone())
        .await?;

    Ok(HttpResponse::Ok().json(updated_application))
}
