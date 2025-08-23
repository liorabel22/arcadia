pub mod search_title_group_info_lite;
use actix_web::web::{get, resource, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/title-group/lite").route(get().to(self::search_title_group_info_lite::exec)),
    );
}
