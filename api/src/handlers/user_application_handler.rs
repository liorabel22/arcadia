use crate::{
    Arcadia, handlers::User,
};
use actix_web::{HttpResponse, web};
use arcadia_storage::{
    models::user_application::{UserApplication, UserApplicationStatus, UserCreatedUserApplication},
    repositories::user_application_repository,
};
use serde::{Deserialize, Serialize};
use arcadia_common::error::{Error, Result};

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct GetUserApplicationsQuery {
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub status: Option<UserApplicationStatus>,
}

#[utoipa::path(
    post,
    path = "/api/apply",
    responses(
        (status = 201, description = "Successfully created user application", body = UserApplication)
    )
)]
pub async fn add_user_application(
    arc: web::Data<Arcadia>,
    application: web::Json<UserCreatedUserApplication>,
) -> Result<HttpResponse> {
    let created_application =
        user_application_repository::create_user_application(arc.pool.borrow(), &application.into_inner())
            .await?;

    Ok(HttpResponse::Created().json(created_application))
}

#[utoipa::path(
    get,
    path = "/api/user-application",
    params(
        ("limit" = Option<i64>, Query, description = "Maximum number of applications to return (default: 50)"),
        ("page" = Option<i64>, Query, description = "Page (default: 1)"),
        ("status" = Option<String>, Query, description = "Filter by application status: 'pending', 'accepted', or 'rejected'"),
        ("checked" = Option<bool>, Query, description = "Filter by checked status: true for checked (accepted/rejected), false for unchecked (pending)")
    ),
    responses(
        (status = 200, description = "Successfully retrieved user applications", body = Vec<UserApplication>),
        (status = 400, description = "Bad Request - Invalid status parameter"),
        (status = 403, description = "Forbidden - Only staff members can view user applications")
    )
)]
pub async fn get_user_applications(
    data: web::Data<Arcadia>,
    user: User,
    query: web::Query<GetUserApplicationsQuery>,
) -> Result<HttpResponse> {
    // Check if user is staff
    if user.class != "staff" {
        return Err(Error::InsufficientPrivileges);
    }

    let applications = user_application_repository::find_user_applications(
        &data.pool.borrow(),
        query.limit.unwrap_or(50),
        query.page.unwrap_or(1),
        query.status.clone(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(applications))
}

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
pub async fn update_user_application_status(
    arc: web::Data<Arcadia>,
    user: User,
    form: web::Json<UpdateUserApplication>,
) -> Result<HttpResponse> {
    // Check if user is staff
    if user.class != "staff" {
        return Err(Error::InsufficientPrivileges);
    }

    let updated_application = user_application_repository::update_user_application_status(
        arc.pool.borrow(),
        form.user_application_id,
        form.status.clone(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(updated_application))
}
