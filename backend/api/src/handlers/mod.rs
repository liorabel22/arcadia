pub mod affiliated_artists;
pub mod announces;
pub mod artists;
pub mod auth;
pub mod conversations;
pub mod edition_groups;
pub mod external_db;
pub mod forum;
pub mod gifts;
pub mod home;
pub mod invitations;
pub mod master_groups;
pub mod search;
pub mod series;
pub mod subscriptions;
pub mod title_groups;
pub mod torrent_requests;
pub mod torrents;
pub mod user_applications;
pub mod users;
pub mod wiki;

pub mod peers_handler;
pub mod scrapers;

use actix_web::{
    dev::Payload, error::ErrorUnauthorized, Error, FromRequest, HttpMessage as _, HttpRequest,
};
use futures_util::future::{err, ok, Ready};

#[derive(Debug, Clone)]
pub struct JwtAuthData {
    pub sub: i64,
}

impl FromRequest for JwtAuthData {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        req.extensions()
            .get::<JwtAuthData>()
            .map(|auth_data| auth_data.clone())
            .map(ok)
            .unwrap_or_else(|| err(ErrorUnauthorized("not authorized")))
    }
}
