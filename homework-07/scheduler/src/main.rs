use std::fs::File;
use std::collections::HashMap;

use serde_yaml::Value;

struct FunctionChoreography {
  workflow_body: Value,
}

fn main() -> anyhow::Result<()> {
  let mut file = File::open("../../project/stock-fc.yml")?;

  let fc: Value = serde_yaml::from_reader(&file)?;
  dbg!(fc);

  Ok(())
}
