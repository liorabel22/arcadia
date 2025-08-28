pub mod create_artists;
pub mod get_artist_publications;

use crate::middlewares::jwt_middleware::authenticate_user;
use actix_web::web::{get, post, resource, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(
                post()
                    .to(self::create_artists::exec)
                    .wrap(HttpAuthentication::with_fn(authenticate_user)),
            )
            .route(get().to(self::get_artist_publications::exec)),
    );
}
