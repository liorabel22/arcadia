use crate::{connection_pool::ConnectionPool, models::notification::NotificationReason};
use arcadia_common::error::{Error, Result};
use sqlx::{Postgres, Transaction};

pub struct NotificationItemsIds {
    pub title_group_id: Option<i64>,
    pub torrent_id: Option<i64>,
    #[allow(dead_code)]
    pub artist_id: Option<i64>,
    #[allow(dead_code)]
    pub collage_id: Option<i64>,
    #[allow(dead_code)]
    pub forum_thread_id: Option<i64>,
}

impl ConnectionPool {
    pub async fn notify_users(
        tx: &mut Transaction<'_, Postgres>,
        reason: &NotificationReason,
        message: Option<&String>,
        notification_items_ids: NotificationItemsIds,
    ) -> Result<()> {
        match reason {
            NotificationReason::TorrentUploadedInSubscribedTitleGroup => {
                sqlx::query!(
                    r#"
                    WITH subscribers_ids AS (
                        SELECT subscriber_id AS user_id
                        FROM subscriptions
                        WHERE title_group_id = $1
                    )
                    INSERT INTO notifications (receiver_id, reason, torrent_id, title_group_id)
                    SELECT
                        user_id,
                        'TorrentUploadedInSubscribedTitleGroup'::notification_reason_enum,
                        $2,
                        $1
                    FROM subscribers_ids
                "#,
                    notification_items_ids.title_group_id,
                    notification_items_ids.torrent_id
                )
                .execute(&mut **tx)
                .await
                .map_err(Error::CouldNotCreateNotification)?;
            }
            NotificationReason::SeedingTorrentDeleted => {
                sqlx::query!(
                    r#"
                    WITH seeders_ids AS (
                        SELECT user_id
                        FROM torrent_activities
                        WHERE torrent_id = $1
                    )
                    INSERT INTO notifications (receiver_id, reason, message, title_group_id)
                    SELECT
                        user_id,
                        'SeedingTorrentDeleted'::notification_reason_enum,
                        $2,
                        $3
                    FROM seeders_ids
                "#,
                    notification_items_ids.torrent_id,
                    message,
                    notification_items_ids.title_group_id
                )
                .execute(&mut **tx)
                .await
                .map_err(Error::CouldNotCreateNotification)?;
            }
            _ => {
                return Err(Error::UnsupportedNotification);
            }
        }

        Ok(())
    }
}
