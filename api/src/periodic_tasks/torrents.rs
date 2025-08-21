use std::sync::Arc;
use arcadia_storage::{connection_pool::ConnectionPool, repositories::torrent_repository};

pub async fn update_torrent_seeders_leechers(pool: Arc<ConnectionPool>) {
    let _ = torrent_repository::update_torrent_seeders_leechers(pool.borrow()).await;
}
