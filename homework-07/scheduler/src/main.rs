use std::fs::File;

mod afcl;
use afcl::FunctionChoreography;

fn main() -> anyhow::Result<()> {
  let file = File::open("../../project/stock-fc.yml")?;

  let fc: FunctionChoreography = serde_yaml::from_reader(&file)?;
  dbg!(&fc);

  fc.to_graph();

  Ok(())
}
