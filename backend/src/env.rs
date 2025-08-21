use std::{collections::HashSet, str::FromStr};
use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Env {
    #[envconfig(nested)]
    pub actix: ActixConfig,
    #[envconfig(from = "POSTGRES_URI")]
    pub postgres_uri: String,
    #[envconfig(from = "JWT_SECRET")]
    pub jwt_secret: String,
    #[envconfig(from = "ARCADIA_OPEN_SIGNUPS")]
    pub open_signups: bool,
    #[envconfig(from = "ARCADIA_FRONTEND_URL")]
    pub frontend_url: String,
    #[envconfig(nested)]
    pub tracker: TrackerConfig,
}

#[derive(Envconfig)]
struct ActixConfig {
    #[envconfig(from = "ACTIX_HOST", default = "127.0.0.1")]
    pub host: String,
    #[envconfig(from = "ACTIX_PORT", default = "8080")]
    pub port: u16,
}

#[derive(Envconfig)]
struct TrackerConfig {
    #[envconfig(from = "ARCADIA_TRACKER_NAME")]
    pub name: String,
    #[envconfig(from = "ARCADIA_TRACKER_URL")]
    pub url: String,
    #[envconfig(from = "ARCADIA_TRACKER_ANNOUNCE_INTERVAL")]
    pub announce_interval: u32,
    #[envconfig(from = "ARCADIA_TRACKER_ANNOUNCE_INTERVAL_GRACE_PERIOD")]
    pub announce_interval_grace_period: u32,
    #[envconfig(from = "ARCADIA_ALLOWED_TORRENT_CLIENTS")]
    pub allowed_torrent_clients: AllowedTorrentClientSet,
    #[envconfig(from = "ARCADIA_GLOBAL_UPLOAD_FACTOR")]
    pub global_upload_factor: f64,
    #[envconfig(from = "ARCADIA_GLOBAL_DOWNLOAD_FACTOR")]
    pub global_download_factor: f64,
    #[envconfig(from = "TASK_INTERVAL_UPDATE_TORRENT_SEEDERS_LEECHERS")]
    pub interval_update_torrent_seeders_leechers: String,
    #[envconfig(from = "TASK_INTERVAL_REMOVE_INACTIVE_PEERS")]
    pub interval_remove_inactive_peers: String,
}

struct AllowedTorrentClientSet {
    clients: HashSet<Vec<u8>>,
}

impl FromStr for AllowedTorrentClientSet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
      let clients = s.split(',')
          .map(|s| s.trim().as_bytes().to_vec())
          .collect::<HashSet<Vec<u8>>>();

      Ok(Self {clients})
    }
}
