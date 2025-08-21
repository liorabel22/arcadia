use crate::{
    Error, Result,
    models::{
        torrent_report::{TorrentReport, UserCreatedTorrentReport},
        user::User,
    },
};
use sqlx::PgPool;

pub async fn report_torrent(
    pool: &PgPool,
    form: &UserCreatedTorrentReport,
    current_user: &User,
) -> Result<TorrentReport> {
    let torrent_report = sqlx::query_as!(
        TorrentReport,
        r#"
            INSERT INTO torrent_reports (reported_by_id, reported_torrent_id, description)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
        current_user.id,
        form.reported_torrent_id,
        form.description,
    )
    .fetch_one(pool)
    .await
    .map_err(Error::CouldNotCreateTorrentReport)?;

    Ok(torrent_report)
}
