pub mod create_torrent_request;
pub mod create_torrent_request_vote;
pub mod fill_torrent_request;
pub mod get_torrent_request;

use crate::middlewares::jwt_middleware::authenticate_user;
use actix_web::web::{get, post, resource, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .wrap(HttpAuthentication::with_fn(authenticate_user))
            .route(post().to(self::create_torrent_request::exec))
            .route(get().to(self::get_torrent_request::exec)),
    );
    cfg.service(
        resource("/fill").route(
            post()
                .to(self::fill_torrent_request::exec)
                .wrap(HttpAuthentication::with_fn(authenticate_user)),
        ),
    );
    cfg.service(
        resource("/vote").route(
            post()
                .to(self::create_torrent_request_vote::exec)
                .wrap(HttpAuthentication::with_fn(authenticate_user)),
        ),
    );
}
