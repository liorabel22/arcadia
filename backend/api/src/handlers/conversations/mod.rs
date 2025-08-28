pub mod create_conversation;
pub mod create_conversation_message;
pub mod get_conversation;

use crate::middlewares::jwt_middleware::authenticate_user;
use actix_web::web::{get, post, resource, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .wrap(HttpAuthentication::with_fn(authenticate_user))
            .route(post().to(self::create_conversation::exec))
            .route(get().to(self::get_conversation::exec)),
    );

    cfg.service(
        resource("/messages").route(
            post()
                .to(self::create_conversation_message::exec)
                .wrap(HttpAuthentication::with_fn(authenticate_user)),
        ),
    );
}
