use actix_web::{HttpResponse, web};

use crate::{
    Arcadia, Result,
    models::{
        torrent_report::{TorrentReport, UserCreatedTorrentReport},
        user::User,
    },
    repositories::torrent_report_repository::report_torrent,
};

#[utoipa::path(
    post,
    path = "/api/report/torrent",
    responses(
        (status = 200, description = "Torrent successfully reported", body=TorrentReport),
    )
)]
pub async fn add_torrent_report(
    form: web::Json<UserCreatedTorrentReport>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let report = report_torrent(&arc.pool, &form, &current_user).await?;

    Ok(HttpResponse::Ok().json(report))
}
