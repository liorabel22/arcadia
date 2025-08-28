pub mod create_wiki_article;
pub mod get_wiki_article;

use crate::middlewares::jwt_middleware::authenticate_user;
use actix_web::web::{get, post, resource, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/articles")
            .route(
                post()
                    .to(self::create_wiki_article::exec)
                    .wrap(HttpAuthentication::with_fn(authenticate_user)),
            )
            .route(get().to(self::get_wiki_article::exec)),
    );
}
