use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use sqlx::FromRow;

use anyhow::Result;

#[derive(Debug)]
pub struct SqlClient {
  pool: MySqlPool
}

#[derive(Debug, FromRow)]
pub struct FunctionImplementation {
  pub name: String,
  pub avg_rtt: f64,
  pub avg_cost: f64,
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

  pub async fn function_type_metadata(&self, function_type: &str) -> Result<Vec<FunctionImplementation>> {
    let function_implementations = sqlx::query_as(r#"
      SELECT functionimplementation.name as name, functionimplementation.avgRTT AS avg_rtt, functionimplementation.avgCost AS avg_cost
      FROM functionimplementation
      JOIN functiontype ON functionimplementation.functionType_id = functiontype.id
      WHERE functiontype.type = ?
    "#).bind(function_type).fetch_all(&self.pool).await?;

    Ok(function_implementations)
  }
}
