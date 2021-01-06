use anyhow::Result;
use serde_json::{json, Value};
use std::env;
use std::io;

mod shared;
use shared::{ProcessResultInput as Input, ProcessResultOutput as Output};

async fn create_chart(params: Value) -> Result<Output> {
  let input: Input = serde_json::from_value(params)?;

  let output = unimplemented!();
  Ok(output)
}

#[tokio::main]
async fn main() -> Result<()> {
  let params: Value = serde_json::from_str(&env::args().nth(1).expect("no argument specified"))?;

  match create_chart(params).await {
    Ok(output) => serde_json::to_writer(io::stdout(), &output)?,
    Err(err) => serde_json::to_writer(io::stdout(), &json!({"error": err.to_string()}))?,
  }

  Ok(())
}
