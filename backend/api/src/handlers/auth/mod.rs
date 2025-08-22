pub mod register;
use actix_web::web::{post, resource, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
  cfg.service(
    resource("/register")
    .route(post().to(self::register::exec))
  );
}
