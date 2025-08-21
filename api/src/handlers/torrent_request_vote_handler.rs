use crate::{handlers::User, Arcadia};
use actix_web::{HttpResponse, web};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::torrent_request_vote::{TorrentRequestVote, UserCreatedTorrentRequestVote},
    repositories::torrent_request_vote_repository::create_torrent_request_vote,
};

#[utoipa::path(
    post,
    path = "/api/torrent-request/vote",
    responses(
        (status = 200, description = "Successfully voted on the torrent_request", body=TorrentRequestVote),
    )
)]
pub async fn add_torrent_request_vote(
    torrent_request_vote: web::Json<UserCreatedTorrentRequestVote>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let vote = create_torrent_request_vote(arc.pool.borrow(), &torrent_request_vote, &current_user).await?;

    Ok(HttpResponse::Created().json(vote))
}
