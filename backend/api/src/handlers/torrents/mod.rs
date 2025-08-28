pub mod create_torrent;
pub mod create_torrent_report;
pub mod delete_torrent;
pub mod download_dottorrent_file;
pub mod edit_torrent;
pub mod get_registered_torrents;
pub mod get_top_torrents;
pub mod get_upload_information;

use crate::middlewares::jwt_middleware::authenticate_user;
use actix_web::web::{delete, get, post, put, resource, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .wrap(HttpAuthentication::with_fn(authenticate_user))
            .route(post().to(self::create_torrent::exec))
            .route(get().to(self::download_dottorrent_file::exec))
            .route(put().to(self::edit_torrent::exec))
            .route(delete().to(self::delete_torrent::exec)),
    );
    cfg.service(
        resource("/registered").route(
            get()
                .to(self::get_registered_torrents::exec)
                .wrap(HttpAuthentication::with_fn(authenticate_user)),
        ),
    );
    cfg.service(
        resource("/upload-info").route(
            get()
                .to(self::get_upload_information::exec)
                .wrap(HttpAuthentication::with_fn(authenticate_user)),
        ),
    );
    cfg.service(resource("/top").route(get().to(self::get_top_torrents::exec)));
    cfg.service(
        resource("/reports").route(
            get()
                .to(self::create_torrent_report::exec)
                .wrap(HttpAuthentication::with_fn(authenticate_user)),
        ),
    );
}
