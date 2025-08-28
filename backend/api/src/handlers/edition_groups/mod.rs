pub mod create_edition_group;

use crate::middlewares::jwt_middleware::authenticate_user;
use actix_web::web::{get, resource, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("").route(
            get()
                .to(self::create_edition_group::exec)
                .wrap(HttpAuthentication::with_fn(authenticate_user)),
        ),
    );
}
