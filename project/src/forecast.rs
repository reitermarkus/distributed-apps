use std::env;
use std::io;

use anyhow::Result;
use serde_json::Value;

fn main() -> Result<()> {
  let params: Value = serde_json::from_str(&env::args().nth(1).unwrap())?;

  let result = params;
  serde_json::to_writer(io::stdout(), &result)?;
  Ok(())
}
