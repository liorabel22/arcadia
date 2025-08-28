pub mod create_series;
pub mod get_series;

use crate::middlewares::jwt_middleware::authenticate_user;
use actix_web::web::{get, post, resource, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(
                post()
                    .to(self::create_series::exec)
                    .wrap(HttpAuthentication::with_fn(authenticate_user)),
            )
            .route(get().to(self::get_series::exec)),
    );
}
