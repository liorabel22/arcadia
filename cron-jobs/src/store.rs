use std::sync::Arc;
use arcadia_storage::connection_pool::ConnectionPool;
use envconfig::Envconfig;
use crate::env::Env;

pub struct Store {
    pub env: Env,
    pub pool: Arc<ConnectionPool>,
}

impl Store {
  pub async fn new() -> Self {
      let env = Env::init_from_env().unwrap();
      let pool = Arc::new(ConnectionPool::try_new(&env.postgres_uri).await.expect("db connection"));

      Self {env, pool}
  }
}
