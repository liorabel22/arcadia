use crate::repositories::announce_repository::{
    credit_user_upload_download, find_torrent_with_id, update_total_seedtime,
};
use crate::repositories::peer_repository::{
    find_torrent_peers, insert_or_update_peer, remove_peer,
};
use crate::repositories::torrent_repository::increment_torrent_completed;
use crate::services::announce_service::is_torrent_client_allowed;
use crate::tracker::announce;
use crate::{Arcadia, repositories::announce_repository::find_user_with_passkey};
use actix_web::{
    FromRequest, HttpRequest, HttpResponse, HttpResponseBuilder, ResponseError, dev, get, web,
};
use serde::Serialize;
use sqlx::types::ipnetwork::IpNetwork;
use std::future::{self, Ready};

trait HttpResponseBuilderExt {
    fn bencode(&mut self, val: impl Serialize) -> HttpResponse;
}

impl HttpResponseBuilderExt for HttpResponseBuilder {
    fn bencode(&mut self, val: impl Serialize) -> HttpResponse {
        match serde_bencode::to_bytes(&val) {
            Ok(data) => self.body(data),
            Err(_) => HttpResponse::BadRequest().body("Failed to bencode"),
        }
    }
}

impl ResponseError for announce::Error {
    fn error_response(&self) -> HttpResponse {
        log::error!("The request generated this error: {self}");
        HttpResponse::BadRequest().body(format!("{self}"))
    }
}

impl FromRequest for announce::Announce {
    type Error = announce::Error;
    type Future = Ready<std::result::Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let query_string = req.query_string();

        let announce = announce::decode_from_query_str(query_string);

        future::ready(announce)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid passkey")]
    InvalidPassKey,

    #[error("invalid info_hash")]
    InvalidInfoHash,

    #[error("invalid user id")]
    InvalidUserId,

    #[error("invalid user id or torrent id")]
    InvalidUserIdOrTorrentId,

    #[error("torrent client not in whitelist")]
    TorrentClientNotInWhitelist,
}

impl actix_web::ResponseError for Error {
    #[inline]
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        log::error!("The request generated this error: {self}");
        #[derive(Debug, Serialize)]
        struct WrappedError {
            #[serde(rename = "failure reason")]
            failure_reason: String,
        }

        actix_web::HttpResponse::build(self.status_code()).bencode(WrappedError {
            failure_reason: self.to_string(),
        })
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct UserAgent(pub String);

impl std::ops::Deref for UserAgent {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserAgentExtractError {
    #[error("no user agent")]
    NoUserAgent,

    #[error("not decodable as utf-8")]
    ToStrError(#[from] actix_web::http::header::ToStrError),
}

impl ResponseError for UserAgentExtractError {
    fn error_response(&self) -> HttpResponse {
        log::error!("The request generated this error: {self}");
        HttpResponse::BadRequest().body(format!("{self}"))
    }
}

impl FromRequest for UserAgent {
    type Error = UserAgentExtractError;
    type Future = Ready<std::result::Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let user_agent = req
            .headers()
            .get("User-Agent")
            .ok_or(UserAgentExtractError::NoUserAgent)
            .and_then(|s| Ok(UserAgent(String::from(s.to_str()?))));

        future::ready(user_agent)
    }
}

#[get("/announce/{passkey}")]
async fn handle_announce(
    arc: web::Data<Arcadia>,
    passkey: web::Path<String>,
    user_agent: Option<UserAgent>,
    ann: announce::Announce,
    conn: dev::ConnectionInfo,
) -> Result<HttpResponse> {
    if !is_torrent_client_allowed(&ann.peer_id, &arc.tracker.allowed_torrent_clients.clients) {
        return Err(Error::TorrentClientNotInWhitelist);
    }

    let passkey = u128::from_str_radix(&passkey, 16).map_err(|_| Error::InvalidPassKey)?;

    let passkey_upper = (passkey >> 64) as i64;
    let passkey_lower = passkey as i64;

    let current_user = find_user_with_passkey(&arc.pool, passkey_upper, passkey_lower).await?;

    let torrent = find_torrent_with_id(&arc.pool, &ann.info_hash).await?;

    let ip = conn
        .realip_remote_addr()
        .and_then(|ip| ip.parse::<IpNetwork>().ok())
        .unwrap();

    if let Some(announce::TorrentEvent::Stopped) = ann.event {
        remove_peer(&arc.pool, &torrent.id, &ann.peer_id, &ip, ann.port).await;
        //return HttpResponse::Ok().into();
        todo!();
    }

    if let Some(announce::TorrentEvent::Completed) = ann.event {
        let _ = increment_torrent_completed(&arc.pool, torrent.id).await;
    }

    let (old_real_uploaded, old_real_downloaded) = insert_or_update_peer(
        &arc.pool,
        &torrent.id,
        &ip,
        &current_user.id,
        &ann,
        user_agent.as_deref(),
    )
    .await;

    let peers = find_torrent_peers(&arc.pool, &torrent.id, &current_user.id).await;

    // assuming that the client either sends both downloaded/uploaded
    // or none of them
    if let (Some(real_uploaded), Some(real_downloaded)) = (ann.uploaded, ann.downloaded) {
        let upload_factor = if arc.tracker.global_upload_factor != 1.0 {
            arc.tracker.global_upload_factor
        } else {
            torrent.upload_factor
        };
        let upload_to_credit = ((real_uploaded as i64 - old_real_uploaded) as f64
            * upload_factor as f64)
            .ceil() as i64;

        let download_factor = if arc.tracker.global_download_factor != 1.0 {
            arc.tracker.global_download_factor
        } else {
            torrent.download_factor
        };
        let download_to_credit = ((real_downloaded as i64 - old_real_downloaded) as f64
            * download_factor as f64)
            .ceil() as i64;
        let real_uploaded_to_credit = real_uploaded as i64 - old_real_uploaded;
        let real_downloaded_to_credit = real_downloaded as i64 - old_real_downloaded;

        // if the client restarted, without sending a "stop" event, keeping the same ip/port
        // calculated upload/download might be negative
        if real_uploaded_to_credit >= 0 && real_downloaded_to_credit >= 0 {
            let _ = credit_user_upload_download(
                &arc.pool,
                upload_to_credit,
                download_to_credit,
                real_uploaded_to_credit,
                real_downloaded_to_credit,
                current_user.id,
            )
            .await;
        }
    }

    if ann.left == Some(0u64) {
        let _ = update_total_seedtime(
            &arc.pool,
            current_user.id,
            torrent.id,
            arc.tracker.announce_interval,
            arc.tracker.announce_interval_grace_period,
        )
        .await;
    }

    let resp = announce::AnnounceResponse {
        peers,
        interval: arc.tracker.announce_interval,
        ..Default::default()
    };

    Ok(HttpResponse::Ok().bencode(resp))
}
