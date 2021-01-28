use anyhow::Result;
use serde_json::{json, Value};
use std::env;
use std::io;
use std::collections::{BTreeMap, HashMap};

mod shared;
use shared::{ProcessResultInput as Input, ProcessResultOutput as Output, Dataset};

mod ibm;
mod macros;
use rusoto_forecastquery::DataPoint;

pub struct SymbolDataPoint {
  pub symbol: String,
  pub value: f64
}

async fn process_result(params: Value) -> Result<Output> {
  let mut input: Input = serde_json::from_value(params)?;
  input.symbols = input.symbols.into_iter().map(|s| s.trim_start_matches("\"").trim_end_matches("\"").to_owned()).collect();
  input.object_keys = input.object_keys.into_iter().map(|s| s.trim_start_matches("\"").trim_end_matches("\"").to_owned()).collect();
  let mut objects: Vec<BTreeMap<String, Vec<DataPoint>>> = Vec::new();

  for o in input.object_keys.iter() {
    let bucket = ibm::bucket().await?;
    let response = try_response!(bucket.get_object(o).await?);
    let response: BTreeMap<String, Vec<DataPoint>> = serde_json::from_slice(&response)?;
    objects.push(response);
  }

  let mut timestamps: BTreeMap<String, HashMap<String, f64>> = BTreeMap::new();

  for (i, mut obj) in objects.into_iter().enumerate() {
    for d in obj.remove("p90").unwrap().into_iter() {
      let symbol = input.symbols[i].to_owned();
      let timestamp = d.timestamp.unwrap();
      let value = d.value.unwrap();

      if let Some(x) = timestamps.get_mut(&timestamp) {
        x.insert(symbol, value);
      } else {
        let mut map = HashMap::new();
        map.insert(symbol, value);
        timestamps.insert(timestamp, map);
      }
    }
  }

  let output = Output {
    labels: timestamps.keys().map(ToOwned::to_owned).collect::<Vec<_>>(),
    datasets: input.symbols.into_iter().map(|symbol| {
      let data = timestamps.values_mut().map(|v| v.remove(&symbol)).collect::<Vec<_>>();

      Dataset {
        label: symbol,
        data
      }
    }).collect::<Vec<_>>(),
  };

  Ok(output)
}

#[tokio::main]
async fn main() -> Result<()> {
  let params: Value = serde_json::from_str(&env::args().nth(1).expect("no argument specified"))?;

  match process_result(params).await {
    Ok(output) => serde_json::to_writer(io::stdout(), &output)?,
    Err(err) => serde_json::to_writer(io::stdout(), &json!({"error": err.to_string()}))?,
  }

  Ok(())
}
