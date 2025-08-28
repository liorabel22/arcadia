pub mod create_subscription;
pub mod remove_subscription;

use crate::middlewares::jwt_middleware::authenticate_user;
use actix_web::web::{delete, post, resource, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .wrap(HttpAuthentication::with_fn(authenticate_user))
            .route(post().to(self::create_subscription::exec))
            .route(delete().to(self::remove_subscription::exec)),
    );
}
