pub mod create_user_application;
pub mod get_user_applications;
pub mod update_user_application_status;

use crate::middlewares::jwt_middleware::authenticate_user;
use actix_web::web::{get, post, put, resource, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_user_application::exec))
            .route(
                get()
                    .to(self::get_user_applications::exec)
                    .wrap(HttpAuthentication::with_fn(authenticate_user)),
            )
            .route(
                put()
                    .to(self::update_user_application_status::exec)
                    .wrap(HttpAuthentication::with_fn(authenticate_user)),
            ),
    );
}
