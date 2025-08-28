pub mod create_forum_post;
pub mod create_forum_thread;
pub mod get_forum;
pub mod get_forum_sub_category_threads;
pub mod get_forum_thread;

use crate::middlewares::jwt_middleware::authenticate_user;
use actix_web::web::{get, post, resource, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(resource("").route(get().to(self::get_forum::exec)));
    cfg.service(
        resource("/thread")
            .route(get().to(self::get_forum_thread::exec))
            .route(
                post()
                    .to(self::create_forum_thread::exec)
                    .wrap(HttpAuthentication::with_fn(authenticate_user)),
            ),
    );
    cfg.service(
        resource("/post").route(
            get()
                .to(self::create_forum_post::exec)
                .wrap(HttpAuthentication::with_fn(authenticate_user)),
        ),
    );
    cfg.service(
        resource("/sub-category").route(get().to(self::get_forum_sub_category_threads::exec)),
    );
}
