use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct ConnectionPool(PgPool);

impl ConnectionPool {
  pub async fn try_new(db_uri: &str) -> Result<Self, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_uri)
        .await
        .expect("Error building a connection pool");

    Ok(Self(pool))
  }

  pub fn borrow(&self) -> &PgPool {
    &self.0
  }

  pub fn borrow_mut(&mut self) -> &mut PgPool {
    &mut self.0
  }
}
