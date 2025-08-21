use sqlx::PgPool;
use eyre::Result;

pub struct ConnectionPool(PgPool);

impl ConnectionPool {
  pub async fn try_new(db_uri: &str) -> Result<Self> {
    let pool = PgPool::connect(db_uri).await?;
    Ok(Self(pool))
  }

  pub fn borrow(&self) -> &PgPool {
    &self.0
  }

  pub fn borrow_mut(&mut self) -> &mut PgPool {
    &mut self.0
  }
}
