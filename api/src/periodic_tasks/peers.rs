use sqlx::PgPool;

use crate::repositories::peer_repository;

pub async fn remove_inactive_peers(
    pool: PgPool,
    announce_interval: u32,
    announce_grace_period: u32,
) {
    let removed_peers_amount = peer_repository::remove_inactive_peers(
        &pool,
        (announce_interval + announce_grace_period) as f64,
    )
    .await;
    log::info!(
        "Removed {} inactive peers from the database",
        removed_peers_amount.unwrap()
    )
}
