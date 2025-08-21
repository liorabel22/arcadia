use std::env;

use actix_web::web;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{Arcadia, periodic_tasks::peers::remove_inactive_peers};

use super::torrents::update_torrent_seeders_leechers;

pub async fn run_periodic_tasks(arc: web::Data<Arcadia>) -> Result<(), Box<dyn std::error::Error>> {
    let sched = JobScheduler::new().await?;

    let arc_for_job1 = arc.clone();
    let update_torrent_seeders_leechers_interval =
        env::var("TASK_INTERVAL_UPDATE_TORRENT_SEEDERS_LEECHERS")
            .expect("env var TASK_INTERVAL_UPDATE_TORRENT_SEEDERS_LEECHERS is missing");
    let job1 = match Job::new_async(
        update_torrent_seeders_leechers_interval.as_str(),
        move |_uuid, _l| Box::pin(update_torrent_seeders_leechers(arc_for_job1.pool.clone())),
    ) {
        Ok(job) => job,
        Err(e) => {
            return Err(format!(
                "Error creating job for updating torrents seeders and leechers: {e}"
            )
            .into());
        }
    };
    sched.add(job1).await?;

    // this interval should be often enough
    // let cleanup_interval_seconds = arc.tracker_announce_interval * 2;
    let remove_inactive_peers_interval = env::var("TASK_INTERVAL_REMOVE_INACTIVE_PEERS")
        .expect("env var TASK_INTERVAL_REMOVE_INACTIVE_PEERS is missing");
    let arc_for_job2 = arc.clone();

    // cleaning old peers is also done when the client sends a "stop" event
    // but it doesn't always do it, so we need to clean the ones that are gone without sending this event
    let job2 = match Job::new_async(remove_inactive_peers_interval.as_str(), move |_uuid, _l| {
        Box::pin(remove_inactive_peers(
            arc_for_job2.pool.clone(),
            arc_for_job2.tracker.announce_interval,
            arc_for_job2.tracker.announce_interval_grace_period,
        ))
    }) {
        Ok(job) => job,
        Err(e) => {
            return Err(format!("Error creating job for cleaning inactive peers: {e}").into());
        }
    };
    sched.add(job2).await?;

    sched.start().await?;

    Ok(())
}
