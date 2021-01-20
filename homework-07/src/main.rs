use std::fs::File;
use std::collections::HashMap;

use dotenv_codegen::dotenv;

mod sql;
use sql::SqlClient;

mod afcl;
use afcl::{FunctionChoreography, Block, LoopCounter, ParallellSection};

fn schedule_parallel_for(block: Block, iterations: usize, concurrency_limit: usize) -> Block {
  if let Block::ParallelFor { name, data_ins, data_outs, loop_counter, loop_body } = &block {
    let mut functions = HashMap::<String, usize>::new();
    let mut concurrency_limits = HashMap::<String, usize>::new();

    Block::Parallel {
      name: name.clone(),
      data_ins: data_ins.clone(),
      parallel_body: vec![ParallellSection { section: vec![Block::ParallelFor {
        name: format!("{}_1", name),
        data_ins: data_ins.clone(), // TODO: map names
        loop_counter: LoopCounter { from: 0.to_string(), to: 0.to_string(), step: loop_counter.step.clone() }, // TODO: map indices
        loop_body: loop_body.clone(),
        data_outs: data_outs.clone(), // TODO: map names
      }] }],
      data_outs: data_outs.clone(),
    }
  } else {
    block
  }
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
  let file = File::open("../project/stock-fc.yml")?;

  let fc: FunctionChoreography = serde_yaml::from_reader(&file)?;
  dbg!(&fc);

  fc.to_graph();
  let user = dotenv!("DB_USER");
  let url = dotenv!("URL");
  let database = dotenv!("DATABASE");
  let password = dotenv!("PASSWORD");
  let sql_client = SqlClient::new(user, password, url, database).await?;
  sql_client.fetch().await?;

  dbg!(&sql_client.function_type_metadata("fetchProcess").await);

  Ok(())
}
