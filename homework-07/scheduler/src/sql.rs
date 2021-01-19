use sqlx::mysql::{MySqlPoolOptions, MySqlDone};
use sqlx::MySqlPool;
use sqlx::Row;

use anyhow::Result;

use futures::TryStreamExt;

#[derive(Debug)]
pub struct SqlClient {
  pool: MySqlPool
}

#[derive(Debug)]
pub struct FunctionTypeMetadata {
  avg_rtt: f64,
  avg_cost: f64,
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

  pub async fn function_type_metadata(&self, function_type: &str) -> Result<FunctionTypeMetadata> {
    let row = sqlx::query(r#"
      SELECT avgRTT, avgCost FROM functiontype
      WHERE name = ?
    "#).bind(function_type).fetch_one(&self.pool).await?;

    let avg_rtt = row.try_get::<Option<_>, _>("avgRTT")?.unwrap_or(0.0);
    let avg_cost = row.try_get::<Option<_>, _>("avgCost")?.unwrap_or(0.0);
    Ok(FunctionTypeMetadata { avg_rtt, avg_cost })
  }
}
