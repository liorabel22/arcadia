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
    #[envconfig(from = "ACTIX_HOST")]
    pub host: String,
    #[envconfig(from = "ACTIX_PORT")]
    pub port: u16,
}

#[derive(Envconfig)]
struct TrackerConfig {
    #[envconfig(from = "ARCADIA_TRACKER_NAME")]
    pub name: String,
    #[envconfig(from = "ARCADIA_TRACKER_URL")]
    pub url: String,
    #[envconfig(from = "ARCADIA_TRACKER_ANNOUNCE_INTERVAL")]
    pub announce_interval: i64,
    #[envconfig(from = "ARCADIA_TRACKER_ANNOUNCE_INTERVAL_GRACE_PERIOD")]
    pub announce_interval_grace_period: i64,
    #[envconfig(from = "ARCADIA_ALLOWED_TORRENT_CLIENTS")]
    pub announce_interval_grace_period: AllowedTorrentClientSet,
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
