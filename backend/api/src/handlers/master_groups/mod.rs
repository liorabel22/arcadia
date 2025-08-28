pub mod create_master_group;

use crate::middlewares::jwt_middleware::authenticate_user;
use actix_web::web::{post, resource, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("").route(
            post()
                .to(self::create_master_group::exec)
                .wrap(HttpAuthentication::with_fn(authenticate_user)),
        ),
    );
}
