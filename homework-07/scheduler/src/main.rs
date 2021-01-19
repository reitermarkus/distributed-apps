use std::fs::File;

use dotenv_codegen::dotenv;

mod sql;
use sql::SqlClient;

mod afcl;
use afcl::FunctionChoreography;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
  let file = File::open("../../project/stock-fc.yml")?;

  let fc: FunctionChoreography = serde_yaml::from_reader(&file)?;
  dbg!(&fc);

  fc.to_graph();
  let user = dotenv!("DB_USER");
  let url = dotenv!("URL");
  let database = dotenv!("DATABASE");
  let password = dotenv!("PASSWORD");

  dbg!(user, url, database, password);

  let sql_client = SqlClient::new(user, password, url, database).await?;
  sql_client.fetch().await?;

  Ok(())
}
