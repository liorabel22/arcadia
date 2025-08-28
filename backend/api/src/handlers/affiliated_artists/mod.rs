pub mod create_affiliated_artists;
pub mod remove_affiliated_artists;

use crate::middlewares::jwt_middleware::authenticate_user;
use actix_web::web::{delete, post, resource, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .wrap(HttpAuthentication::with_fn(authenticate_user))
            .route(post().to(self::create_affiliated_artists::exec))
            .route(delete().to(self::remove_affiliated_artists::exec)),
    );
}
