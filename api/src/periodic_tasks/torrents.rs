use sqlx::PgPool;

use crate::repositories::torrent_repository;

pub async fn update_torrent_seeders_leechers(pool: PgPool) {
    let _ = torrent_repository::update_torrent_seeders_leechers(&pool).await;
}
