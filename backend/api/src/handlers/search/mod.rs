pub mod search_title_group_info_lite;
pub mod search_torrent_requests;
pub mod search_torrents;

use actix_web::web::{get, resource, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/title-group/lite").route(get().to(self::search_title_group_info_lite::exec)),
    );
    cfg.service(resource("/torrent/lite").route(get().to(self::search_torrents::exec)));
    cfg.service(resource("/torrent-requests").route(get().to(self::search_torrent_requests::exec)));
}
