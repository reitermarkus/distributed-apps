use sqlx::mysql::{MySqlPoolOptions, MySqlDone};
use sqlx::MySqlPool;

use anyhow::Result;

use futures::TryStreamExt;

pub struct SqlClient {
  pool: MySqlPool
}

impl SqlClient {
  pub async fn new(user: &str, password: &str, url: &str, database: &str) -> Result<SqlClient> {
    let pool = MySqlPoolOptions::new()
      .max_connections(5)
      .connect(&format!("mysql://{}:{}@{}/{}", user, password, url, database)).await?;

    Ok(SqlClient {
      pool: pool
    })
  }

  pub async fn fetch(&self) -> Result<()> {
    let mut rows = sqlx::query("SELECT * FROM FCdeployment").fetch(&self.pool);

    while let Some(row) = rows.try_next().await? {
      dbg!(row);
    }

    Ok(())
  }
}
