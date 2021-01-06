use anyhow::Result;
use serde_json::{json, Value};
use std::env;
use std::io;
use std::collections::BTreeMap;

mod shared;
use shared::{ProcessResultInput as Input, ProcessResultOutput as Output, Dataset};

mod ibm;
mod macros;
use rusoto_forecastquery::DataPoint;

pub struct SymbolDataPoint {
  pub symbol: String,
  pub timestamp: String,
  pub value: f64
}


async fn process_results(params: Value) -> Result<Output> {
  let input: Input = serde_json::from_value(params)?;
  let mut objects: Vec<BTreeMap<String, Vec<DataPoint>>> = Vec::new();

  for o in input.object_keys.iter() {
    let bucket = ibm::bucket().await?;
    let response = try_response!(bucket.get_object(o).await?);
    let response: BTreeMap<String, Vec<DataPoint>> = serde_json::from_slice(&response)?;
    objects.push(response);
  }

  let mut timestamps: Vec<SymbolDataPoint> = Vec::new();

  for obj in objects.iter() {
    let p90 : &Vec<DataPoint> = obj.get("p90").unwrap();
    for (p, s) in p90.iter().zip(input.symbols.iter()) {
      timestamps.push(SymbolDataPoint {
        symbol: s.to_owned(),
        timestamp: p.timestamp.clone().unwrap(),
        value: p.value.unwrap()
      });
    }
  }

  let output = Output {
    labels: timestamps.iter().map(|t| t.symbol).collect::<Vec<_>>(),
    datasets: timestamps.iter().map(|t| Dataset {
      label: t.timestamp,
      data: t.value
    }).collect::<Vec<_>>()
  };

  Ok(output)
}

#[tokio::main]
async fn main() -> Result<()> {
  let params: Value = serde_json::from_str(&env::args().nth(1).expect("no argument specified"))?;

  match process_results(params).await {
    Ok(output) => serde_json::to_writer(io::stdout(), &output)?,
    Err(err) => serde_json::to_writer(io::stdout(), &json!({"error": err.to_string()}))?,
  }

  Ok(())
}
