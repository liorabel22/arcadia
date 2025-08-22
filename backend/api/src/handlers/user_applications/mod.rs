pub mod create_user_application;
pub mod get_user_applications;
pub mod update_user_application_status;

use actix_web::web::{ServiceConfig, get, post, put, resource};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_user_application::exec))
            .route(get().to(self::get_user_applications::exec))
            .route(put().to(self::update_user_application_status::exec)),
    );
}
