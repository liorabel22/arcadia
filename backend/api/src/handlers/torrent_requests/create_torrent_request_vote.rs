use crate::{middlewares::jwt_middleware::JwtAuthData, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::torrent_request_vote::{
    TorrentRequestVote, UserCreatedTorrentRequestVote,
};

#[utoipa::path(
    post,
    operation_id = "Create torrent request vote",
    tag = "Torrent Request",
    path = "/api/torrent-requests/vote",
    responses(
        (status = 200, description = "Successfully voted on the torrent_request", body=TorrentRequestVote),
    )
)]
pub async fn exec(
    torrent_request_vote: web::Json<UserCreatedTorrentRequestVote>,
    arc: web::Data<Arcadia>,
    user: JwtAuthData,
) -> Result<HttpResponse> {
    let vote = arc
        .pool
        .create_torrent_request_vote(&torrent_request_vote, user.sub)
        .await?;

    Ok(HttpResponse::Created().json(vote))
}
