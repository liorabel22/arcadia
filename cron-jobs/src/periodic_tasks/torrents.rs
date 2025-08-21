use std::sync::Arc;
use arcadia_storage::connection_pool::ConnectionPool;

pub async fn update_torrent_seeders_leechers(pool: Arc<ConnectionPool>) {
    let _ = pool.update_torrent_seeders_leechers().await;
}
